//! Managing functions. These functions wrap the functions from LHAPDF that mail fail due to data
//! not being downloaded. In that case we do the best to download them from locations and to a
//! directory specified in our configuration file.

use super::config::Config;
use super::ffi::{self, PDFSet, PDF};
use super::unmanaged;
use super::{Error, Result};
use cxx::UniquePtr;
use flate2::read::GzDecoder;
use reqwest::blocking;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tar::Archive;

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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

struct LhapdfData;

impl LhapdfData {
    fn get() -> &'static Mutex<LhapdfData> {
        static SINGLETON: Mutex<LhapdfData> = Mutex::new(LhapdfData);
        &SINGLETON
    }

    fn download_set(&self, name: &str, config: &Config) -> Result<()> {
        // TODO: this function has a race condition if there multiple processes (not multiple
        // threads) that try to create the same file

        for url in config.pdfset_urls() {
            let response = blocking::get(format!("{url}/{name}.tar.gz"))?;

            if response.status() == StatusCode::NOT_FOUND {
                continue;
            }

            let content = response.bytes()?;

            // TODO: what if multiple threads/processes try to write to the same file?
            Archive::new(GzDecoder::new(&content[..])).unpack(config.lhapdf_data_path_write())?;

            // we found a PDF set, now it's LHAPDF's turn
            break;
        }

        Ok(())
    }

    fn update_pdfsets_index(&self, config: &Config) -> Result<()> {
        // TODO: this function has a race condition if there multiple processes (not multiple
        // threads) that try to create the same file

        // empty the `static thread_local` variable sitting in `getPDFIndex` to trigger the
        // re-initialization of this variable
        ffi::empty_lhaindex();

        // download `pdfsets.index`
        let content = blocking::get(config.pdfsets_index_url())?.text()?;

        let pdfsets_index = PathBuf::from(config.lhapdf_data_path_write()).join("pdfsets.index");

        // TODO: what if multiple threads/processes try to write to the same file?
        File::create(pdfsets_index)?.write_all(content.as_bytes())?;

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
        unmanaged::set_verbosity(verbosity)
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
