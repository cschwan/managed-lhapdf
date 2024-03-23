use super::ffi::{self, PDFSet, PDF};
use super::{LhapdfError, Result};
use cxx::{let_cxx_string, UniquePtr};

pub fn pdf_name_and_member_via_lhaid(lhaid: i32) -> Option<(String, i32)> {
    let_cxx_string!(cxx_setname = "");
    ffi::lookup_pdf_setname(lhaid, cxx_setname.as_mut());

    let setname = cxx_setname.to_string_lossy();
    let memberid = ffi::lookup_pdf_memberid(lhaid);

    if (setname == "") && (memberid == -1) {
        None
    } else {
        Some((setname.to_string(), memberid))
    }
}

pub fn pdf_with_lhaid(lhaid: i32) -> Result<UniquePtr<PDF>> {
    ffi::pdf_with_lhaid(lhaid).map_err(|exc| LhapdfError { exc })
}

pub fn pdf_with_set_and_member(set: &UniquePtr<PDFSet>, member: i32) -> Result<UniquePtr<PDF>> {
    ffi::pdf_with_set_and_member(set, member).map_err(|exc| LhapdfError { exc })
}

pub fn pdf_with_setname_and_member(setname: &str, member: i32) -> Result<UniquePtr<PDF>> {
    let_cxx_string!(cxx_setname = setname.to_string());
    ffi::pdf_with_setname_and_member(&cxx_setname, member).map_err(|exc| LhapdfError { exc })
}

pub fn pdf_with_setname_and_nmem(setname_nmem: &str) -> Result<UniquePtr<PDF>> {
    let_cxx_string!(cxx_setname = setname_nmem.to_string());
    ffi::pdf_with_setname_and_nmem(&cxx_setname).map_err(|exc| LhapdfError { exc })
}

pub fn pdfset_from_pdf(pdf: &UniquePtr<PDF>) -> UniquePtr<PDFSet> {
    ffi::pdfset_from_pdf(pdf)
}

pub fn pdfset_new(setname: &str) -> Result<UniquePtr<PDFSet>> {
    let_cxx_string!(cxx_setname = setname);

    ffi::pdfset_new(&cxx_setname).map_err(|exc| LhapdfError { exc })
}