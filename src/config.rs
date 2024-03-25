use super::{Error, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::sync::OnceLock;

/// Configuration for this library.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    lhapdf_data_path_read: Vec<String>,
    lhapdf_data_path_write: String,
    pdfsets_index_url: String,
    pdfset_urls: Vec<String>,
}

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
                            .join("managed-lhapdf")
                            .to_str()
                            // UNWRAP: if the string isn't valid unicode we can't proceed
                            .unwrap()
                            .to_owned(),
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
                            os_str.to_str().unwrap().split(':').map(ToOwned::to_owned).collect();
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

            // we use the environment variable `LHAPDF_DATA_PATH` to let LHAPDF know where we've
            // stored our PDFs

            let mut lhapdf_data_path = vec![config.lhapdf_data_path_write.clone()];
            lhapdf_data_path.extend(config.lhapdf_data_path_read.iter().cloned());
            // as long as `static Config _cfg` in LHAPDF's `src/Config.cc` is `static` and not
            // `thread_local`, this belongs here; otherwise move it out of the singleton
            // initialization
            env::set_var("LHAPDF_DATA_PATH", lhapdf_data_path.join(":"));

            Ok(config)
        });

        // TODO: change return type and propagate the result - difficult because we can't clone the
        // error type
        config.as_ref().unwrap()
    }

    /// Return the path where `managed-lhapdf` will download PDF sets and `pdfsets.index` to.
    pub fn lhapdf_data_path_write(&self) -> &str {
        &self.lhapdf_data_path_write
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
