//! Managing functions. These functions wrap the functions from LHAPDF that mail fail due to data
//! not being downloaded. In that case we do the best to download them from locations and to a
//! directory specified in our configuration file.

use super::config::Config;
use super::ffi::{self, PDFSet, PDF};
use super::{Error, Result};
use cxx::{let_cxx_string, UniquePtr};
use flate2::read::GzDecoder;
use reqwest::blocking;
use reqwest::StatusCode;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tar::Archive;

fn download_set(name: &str, config: &Config) -> Result<()> {
    for url in config.pdfset_urls() {
        let response = blocking::get(format!("{url}/{name}.tar.gz"))?;

        if response.status() == StatusCode::NOT_FOUND {
            continue;
        }

        let content = response.bytes()?;

        // download directory may not exist
        fs::create_dir_all(config.lhapdf_data_path_write())?;

        // TODO: what if multiple threads/processes try to write to the same file?
        Archive::new(GzDecoder::new(&content[..])).unpack(config.lhapdf_data_path_write())?;

        // we found a PDF set, now it's LHAPDF's turn
        break;
    }

    Ok(())
}

fn update_pdfsets_index(config: &Config) -> Result<()> {
    // empty the `static thread_local` variable sitting in `getPDFIndex` to trigger the
    // re-initialization of this variable
    ffi::empty_lhaindex();

    // download `pdfsets.index`
    let content = blocking::get(config.pdfsets_index_url())?.text()?;

    // download directory may not exist
    fs::create_dir_all(config.lhapdf_data_path_write())?;

    let pdfsets_index = PathBuf::from(config.lhapdf_data_path_write()).join("pdfsets.index");

    // TODO: what if multiple threads/processes try to write to the same file?
    File::create(pdfsets_index)?.write_all(content.as_bytes())?;

    let _ = config.lhapdf_data_path_write();
    let _ = config.pdfsets_index_url();

    Ok(())
}

pub fn pdf_name_and_member_via_lhaid(lhaid: i32) -> Option<(String, i32)> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    let callable = || {
        let_cxx_string!(cxx_setname = "");
        ffi::lookup_pdf_setname(lhaid, cxx_setname.as_mut());

        // UNWRAP: if `setname` contains any non-UTF8 bytes there's an error somewhere else
        let setname = cxx_setname.to_str().unwrap();
        let memberid = ffi::lookup_pdf_memberid(lhaid);

        if setname.is_empty() && (memberid == -1) {
            None
        } else {
            Some((setname.to_owned(), memberid))
        }
    };

    callable().or_else(|| {
        // TODO: change return type of this function and handle the error properly
        update_pdfsets_index(config).unwrap();
        callable()
    })
}

pub fn pdf_with_setname_and_member(setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    let_cxx_string!(cxx_setname = setname.to_string());

    let callable =
        || ffi::pdf_with_setname_and_member(&cxx_setname, member).map_err(Error::LhapdfException);

    callable().or_else(|err| {
        // here we rely on exactly matching LHAPDF's exception string
        if err.to_string() == format!("Info file not found for PDF set '{setname}'") {
            download_set(setname, config).and_then(|()| callable())
        } else {
            Err(err)
        }
    })
}

pub fn pdfset_new(setname: &str) -> Result<UniquePtr<PDFSet>> {
    // this must be the first call before anything from LHAPDF
    let config = Config::get();

    let_cxx_string!(cxx_setname = setname);

    let callable = || ffi::pdfset_new(&cxx_setname).map_err(Error::LhapdfException);

    callable().or_else(|err| {
        // here we rely on exactly matching LHAPDF's exception string
        if err.to_string() == format!("Info file not found for PDF set '{setname}'") {
            download_set(setname, config).and_then(|()| callable())
        } else {
            Err(err)
        }
    })
}
