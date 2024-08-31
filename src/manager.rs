//! Managing functions. These functions wrap the functions from LHAPDF that mail fail due to data
//! not being downloaded. In that case we do the best to download them from locations and to a
//! directory specified in our configuration file.

use super::ffi::{self, PDFSet, PDF};
use super::unmanaged;
use super::{Error, Result};
use cxx::UniquePtr;
use flate2::read::GzDecoder;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io;
use std::io::{ErrorKind, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tar::Archive;

const LHAPDF_CONFIG: &str = "Verbosity: 1
Interpolator: logcubic
Extrapolator: continuation
ForcePositive: 0
AlphaS_Type: analytic
MZ: 91.1876
MUp: 0.002
MDown: 0.005
MStrange: 0.10
MCharm: 1.29
MBottom: 4.19
MTop: 172.9
Pythia6LambdaV5Compat: true
";

/// Configuration for this library.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    lhapdf_data_path_read: Vec<PathBuf>,
    lhapdf_data_path_write: PathBuf,
    pdfsets_index_url: String,
    pdfset_urls: Vec<String>,
}

struct LhapdfData;

impl Config {
    /// Return the only instance of this type.
    pub fn get() -> &'static Self {
        static SINGLETON: OnceLock<Result<Config>> = OnceLock::new();

        let config = SINGLETON.get_or_init(|| {
            let config_path = dirs::config_dir()
                .ok_or_else(|| Error::General("no configuration directory found".to_owned()))?;

            // create the configuration directory if it doesn't exist yet - in practice this only
            // happens in our CI
            fs::create_dir_all(&config_path)?;

            let config_path = config_path.join("managed-lhapdf.toml");

            // TODO: it's possible that multiple processes try to create the default configuration
            // file and/or that while the file is created, other processes try to read from it

            // MSRV 1.77.0: use `File::create_new` instead
            let config = match File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&config_path)
            {
                // the file didn't exist before
                Ok(mut file) => {
                    // use a default configuration
                    let mut config = Self {
                        lhapdf_data_path_read: vec![],
                        lhapdf_data_path_write: dirs::data_dir()
                            .ok_or_else(|| Error::General("no data directory found".to_owned()))?
                            .join("managed-lhapdf"),
                        pdfsets_index_url: "https://lhapdfsets.web.cern.ch/current/pdfsets.index"
                            .to_owned(),
                        pdfset_urls: vec!["https://lhapdfsets.web.cern.ch/current/".to_owned()],
                    };

                    // if there's an environment variable that the user set use its value
                    if let Some(os_str) =
                        env::var_os("LHAPDF_DATA_PATH").or_else(|| env::var_os("LHAPATH"))
                    {
                        config.lhapdf_data_path_read =
                            // UNWRAP: if the string isn't valid unicode we can't proceed
                            os_str.to_str().unwrap().split(':').map(PathBuf::from).collect();
                    }

                    file.write_all(toml::to_string_pretty(&config)?.as_bytes())?;

                    config
                }
                Err(err) if err.kind() == ErrorKind::AlreadyExists => {
                    // the file already exists, simply read it
                    toml::from_str(&fs::read_to_string(&config_path)?)?
                }
                Err(err) => Err(err)?,
            };

            if let Some(lhapdf_data_path_write) = config.lhapdf_data_path_write() {
                // create download directory for `lhapdf.conf`
                fs::create_dir_all(lhapdf_data_path_write)?;

                // MSRV 1.77.0: use `File::create_new` instead
                if let Ok(mut file) = File::options()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(lhapdf_data_path_write.join("lhapdf.conf"))
                {
                    // if `lhapdf.conf` doesn't exist, create it
                    file.write_all(LHAPDF_CONFIG.as_bytes())?;
                }

                let pdfsets_index = lhapdf_data_path_write.join("pdfsets.index");

                // MSRV 1.77.0: use `File::create_new` instead
                if let Ok(mut file) = File::options()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(pdfsets_index)
                {
                    // if `pdfsets.index` doesn't exist, download it
                    let mut reader = ureq::get(config.pdfsets_index_url()).call()?.into_reader();
                    io::copy(&mut reader, &mut file)?;
                }
            }

            // we use the environment variable `LHAPDF_DATA_PATH` to let LHAPDF know where we've
            // stored our PDFs

            let lhapdf_data_path = config
                .lhapdf_data_path_write()
                .into_iter()
                .chain(config.lhapdf_data_path_read.iter().map(Deref::deref))
                .map(|path| path.as_os_str())
                .collect::<Vec<_>>()
                .join(&OsString::from(":"));
            // as long as `static Config _cfg` in LHAPDF's `src/Config.cc` is `static` and not
            // `thread_local`, this belongs here; otherwise move it out of the singleton
            // initialization
            env::set_var("LHAPDF_DATA_PATH", lhapdf_data_path);

            Ok(config)
        });

        // TODO: change return type and propagate the result - difficult because we can't clone the
        // error type
        config.as_ref().unwrap()
    }

    /// Return the path where `managed-lhapdf` will download PDF sets and `pdfsets.index` to.
    pub fn lhapdf_data_path_write(&self) -> Option<&Path> {
        if self.lhapdf_data_path_write.as_os_str().is_empty() {
            None
        } else {
            Some(&self.lhapdf_data_path_write)
        }
    }

    /// Return the URL where the file `pdfsets.index` will downloaded from.
    pub fn pdfsets_index_url(&self) -> &str {
        &self.pdfsets_index_url
    }

    /// Return the URLs that should be searched for PDF sets, if they are not available in the
    /// local cache.
    pub fn pdfset_urls(&self) -> &[String] {
        &self.pdfset_urls
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

impl LhapdfData {
    fn get() -> &'static Mutex<Self> {
        static SINGLETON: Mutex<LhapdfData> = Mutex::new(LhapdfData);
        &SINGLETON
    }

    fn download_set(&self, name: &str, config: &Config) -> Result<()> {
        if let Some(lhapdf_data_path_write) = config.lhapdf_data_path_write() {
            let lock_file = File::create(lhapdf_data_path_write.join(format!("{name}.lock")))?;
            lock_file.lock_exclusive()?;

            for url in config.pdfset_urls() {
                let response = ureq::get(&format!("{url}/{name}.tar.gz")).call();

                if let Err(ureq::Error::Status(404, _)) = response {
                    continue;
                }

                let reader = response?.into_reader();

                // TODO: what if multiple threads/processes try to write to the same file?
                Archive::new(GzDecoder::new(reader)).unpack(lhapdf_data_path_write)?;

                // we found a PDF set, now it's LHAPDF's turn
                break;
            }

            lock_file.unlock()?;
        }

        Ok(())
    }

    fn update_pdfsets_index(&self, config: &Config) -> Result<()> {
        if let Some(lhapdf_data_path_write) = config.lhapdf_data_path_write() {
            let lock_file = File::create(lhapdf_data_path_write.join("pdfsets.lock"))?;
            lock_file.lock_exclusive()?;

            // empty the `static thread_local` variable sitting in `getPDFIndex` to trigger the
            // re-initialization of this variable
            ffi::empty_lhaindex();

            // download `pdfsets.index`
            let content = ureq::get(config.pdfsets_index_url())
                .call()?
                .into_string()?;

            let pdfsets_index = lhapdf_data_path_write.join("pdfsets.index");

            // TODO: what if multiple threads/processes try to write to the same file?
            File::create(pdfsets_index)?.write_all(content.as_bytes())?;

            lock_file.unlock()?;
        }

        Ok(())
    }

    pub fn pdf_name_and_member_via_lhaid(&self, lhaid: i32) -> Option<(String, i32)> {
        unmanaged::pdf_name_and_member_via_lhaid(lhaid)
    }

    fn pdf_with_setname_and_member(&self, setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
        unmanaged::pdf_with_setname_and_member(setname, member)
    }

    fn pdfset_new(&self, setname: &str) -> Result<UniquePtr<PDFSet>> {
        unmanaged::pdfset_new(setname)
    }

    fn set_verbosity(&self, verbosity: i32) {
        unmanaged::set_verbosity(verbosity);
    }

    fn verbosity(&self) -> i32 {
        unmanaged::verbosity()
    }
}

pub fn pdf_name_and_member_via_lhaid(lhaid: i32) -> Option<(String, i32)> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    // TODO: change return type of this function and handle the error properly
    let lock = LhapdfData::get().lock().unwrap();

    lock.pdf_name_and_member_via_lhaid(lhaid).or_else(|| {
        // TODO: change return type of this function and handle the error properly
        lock.update_pdfsets_index(config).unwrap();
        lock.pdf_name_and_member_via_lhaid(lhaid)
    })
}

pub fn pdf_with_setname_and_member(setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    // TODO: handle error properly
    let lock = LhapdfData::get().lock().unwrap();

    lock.pdf_with_setname_and_member(setname, member)
        .or_else(|err: Error| {
            // here we rely on exactly matching LHAPDF's exception string
            if err.to_string() == format!("Info file not found for PDF set '{setname}'") {
                lock.download_set(setname, config)
                    .and_then(|()| lock.pdf_with_setname_and_member(setname, member))
            } else {
                Err(err)
            }
        })
}

pub fn pdfset_new(setname: &str) -> Result<UniquePtr<PDFSet>> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    // TODO: handle error properly
    let lock = LhapdfData::get().lock().unwrap();

    lock.pdfset_new(setname).or_else(|err: Error| {
        // here we rely on exactly matching LHAPDF's exception string
        if err.to_string() == format!("Info file not found for PDF set '{setname}'") {
            lock.download_set(setname, config)
                .and_then(|()| lock.pdfset_new(setname))
        } else {
            Err(err)
        }
    })
}

pub fn set_verbosity(verbosity: i32) {
    // this must be the first call before anything from LHAPDF
    let _ = Config::get();

    // TODO: handle error properly
    let lock = LhapdfData::get().lock().unwrap();

    lock.set_verbosity(verbosity);
}

pub fn verbosity() -> i32 {
    // this must be the first call before anything from LHAPDF
    let _ = Config::get();

    // TODO: handle error properly
    let lock = LhapdfData::get().lock().unwrap();

    lock.verbosity()
}
