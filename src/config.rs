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
    lhapdf_data_path: Vec<String>,
    repositories: Vec<String>,
}

impl Config {
    /// Return the only instance of this type.
    pub fn get() -> &'static Self {
        static SINGLETON: OnceLock<Result<Config>> = OnceLock::new();

        let config = SINGLETON.get_or_init(|| {
            let config_path = dirs::config_dir()
                .ok_or_else(|| Error::General(format!("no configuration directory found")))?;

            // create the configuration directory if it doesn't exist yet - in practice this only
            // happens in our CI
            fs::create_dir_all(&config_path).map_err(|err| Error::General(format!("{err}")))?;

            let config_path = config_path.join("managed-lhapdf.toml");

            // MSRV 1.77.0: use `File::create_new` instead
            let config = match File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&config_path)
            {
                Ok(mut file) => {
                    // the file didn't exist, use the default configuration ...
                    let mut config = Config {
                        lhapdf_data_path: vec![dirs::data_dir()
                            .ok_or_else(|| Error::General(format!("no data directory found")))?
                            .join("managed-lhapdf")
                            .to_str()
                            // UNWRAP: if the string isn't valid unicode we can't proceed
                            .unwrap()
                            .to_owned()],
                        repositories: vec!["https://lhapdfsets.web.cern.ch/current/".to_owned()],
                    };

                    // if there's an environment variable that the user set use its value
                    if let Some(os_str) =
                        env::var_os("LHAPDF_DATA_PATH").or_else(|| env::var_os("LHAPATH"))
                    {
                        config.lhapdf_data_path =
                            // UNWRAP: if the string isn't valid unicode we can't proceed
                            os_str.to_str().unwrap().split(':').map(ToOwned::to_owned).collect();
                    }

                    // and write it to the file we've just created
                    file.write_all(
                        &toml::to_string_pretty(&config)
                            .map_err(|err| Error::General(format!("{err}")))?
                            .as_bytes(),
                    )
                    .map_err(|err| Error::General(format!("{err}")))?;

                    config
                }
                Err(err) if err.kind() == ErrorKind::AlreadyExists => {
                    // the file already exists, simply read it
                    toml::from_str(
                        &fs::read_to_string(&config_path)
                            .map_err(|err| Error::General(format!("{err}")))?,
                    )
                    .map_err(|err| Error::General(format!("{err}")))?
                }
                Err(err) => return Err(Error::General(format!("{err}"))),
            };

            // we use the environment variable `LHAPDF_DATA_PATH` to let LHAPDF know where we've
            // stored our PDFs

            // as long as `static Config _cfg` in LHAPDF's `src/Config.cc` is `static` and not
            // `thread_local`, this belongs here; otherwise move it out of the singleton
            // initialization
            env::set_var("LHAPDF_DATA_PATH", config.lhapdf_data_path.join(":"));

            Ok(config)
        });

        // TODO: change return type and propagate the result - difficult because we can't clone the
        // error type
        config.as_ref().unwrap()
    }

    /// Return the URLs that should be searched for PDF sets, if they are not available in the
    /// local cache.
    pub fn repositories(&self) -> &[String] {
        &self.repositories
    }
}
