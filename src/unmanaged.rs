use super::ffi::{self, PDFSet, PDF};
use super::Result;
use cxx::{let_cxx_string, UniquePtr};

pub fn pdf_name_and_member_via_lhaid(lhaid: i32) -> Option<(String, i32)> {
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
}

pub fn pdf_with_setname_and_member(setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
    let_cxx_string!(cxx_setname = setname.to_string());
    Ok(ffi::pdf_with_setname_and_member(&cxx_setname, member)?)
}

pub fn pdfset_new(setname: &str) -> Result<UniquePtr<PDFSet>> {
    let_cxx_string!(cxx_setname = setname);
    Ok(ffi::pdfset_new(&cxx_setname)?)
}

pub fn set_verbosity(verbosity: i32) {
    // this modifies a `static` variable in C++, beware of threads calling this function at the
    // same time
    ffi::setVerbosity(verbosity);
}

pub fn verbosity() -> i32 {
    // accesses a `static` variable in C++
    ffi::verbosity()
}
