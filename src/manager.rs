//! Managing functions. These functions wrap the functions from LHAPDF that mail fail due to data
//! not being downloaded. In that case we do the best to download them from locations and to a
//! directory specified in our configuration file.

use super::ffi::{self, PDFSet, PDF};
use super::{Error, Result};
use cxx::{let_cxx_string, UniquePtr};

fn download_set(_name: &str) -> Result<()> {
    // TODO: try to find and download the pdf set from one of the repositories into
    // `lhapdf_data_path`
    Ok(())
}

fn update_pdfsets_index() -> Option<()> {
    // empty the `static thread_local` variable sitting in `getPDFIndex` to trigger the
    // re-initialization of this variable
    ffi::empty_lhaindex();

    // TODO: download updated `pdfsets.index`

    Some(())
}

pub fn pdf_name_and_member_via_lhaid(lhaid: i32) -> Option<(String, i32)> {
    let callable = || {
        let_cxx_string!(cxx_setname = "");
        ffi::lookup_pdf_setname(lhaid, cxx_setname.as_mut());

        // UNWRAP: if `setname` contains any non-UTF8 bytes there's an error somewhere else
        let setname = cxx_setname.to_str().unwrap();
        let memberid = ffi::lookup_pdf_memberid(lhaid);

        if (setname == "") && (memberid == -1) {
            None
        } else {
            Some((setname.to_owned(), memberid))
        }
    };

    callable().or_else(|| update_pdfsets_index().and_then(|_| callable()))
}

pub fn pdf_with_setname_and_member(setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
    let_cxx_string!(cxx_setname = setname.to_string());

    let callable =
        || ffi::pdf_with_setname_and_member(&cxx_setname, member).map_err(Error::LhapdfException);

    callable().or_else(|err| {
        // here we rely on exactly matching LHAPDF's exception string
        if err.to_string() == format!("Can't find a valid PDF {setname}/{member}") {
            download_set(setname).and_then(|_| callable())
        } else {
            Err(err)
        }
    })
}

pub fn pdfset_new(setname: &str) -> Result<UniquePtr<PDFSet>> {
    let_cxx_string!(cxx_setname = setname);

    let callable = || ffi::pdfset_new(&cxx_setname).map_err(Error::LhapdfException);

    callable().or_else(|err| {
        // here we rely on exactly matching LHAPDF's exception string
        if err.to_string() == format!("Info file not found for PDF set '{setname}'") {
            download_set(setname).and_then(|_| callable())
        } else {
            Err(err)
        }
    })
}
