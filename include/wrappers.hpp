#ifndef WRAPPERS_HPP
#define WRAPPERS_HPP

// This file defines a few wrappers on top of the real/fake LHAPDF library, which are needed due to
// the limitation of the `cxx` crate: we can't use C++ functions that return `std::string` or
// `std::pair` and constructors need to be wrapped with a function returning the constructed object
// in a `std::unique_ptr`. Finally, LHAPDF decided to change to layout of the type `PDFUncertainty`,
// so to be safe against those changes we define our own type.

#ifdef FAKE_WRAPPERS
#include "fake-lhapdf.hpp"
#else
#include <LHAPDF/LHAPDF.h>
#endif

#include <managed-lhapdf/src/lib.rs.h>
#include <rust/cxx.h>

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

inline void pdf_setname(LHAPDF::PDF const& pdf, std::string& name) {
    name = pdf.set().name();
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_setname_and_member(
    std::string const& setname,
    std::int32_t member
) {
    return std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(setname, member));
}

inline std::unique_ptr<LHAPDF::PDFSet> pdfset_new(std::string const& setname) {
    return std::unique_ptr<LHAPDF::PDFSet>(new LHAPDF::PDFSet(setname));
}

inline void pdfset_setname(LHAPDF::PDFSet const& pdfset, std::string& name) {
    name = pdfset.name();
}

inline void lookup_pdf_setname(std::int32_t lhaid, std::string& setname) {
    setname = LHAPDF::lookupPDF(lhaid).first;
}

inline std::int32_t lookup_pdf_memberid(std::int32_t lhaid) {
    return LHAPDF::lookupPDF(lhaid).second;
}

inline void get_pdfset_error_type(LHAPDF::PDFSet const& set, std::string& error_type) {
    error_type = set.errorType();
}

inline PdfUncertainty pdf_uncertainty(
    LHAPDF::PDFSet const& pdfset,
    rust::Slice<double const> values,
    double cl,
    bool alternative
) {
    std::vector<double> const vector(values.begin(), values.end());
    auto const uncertainty = pdfset.uncertainty(vector, cl, alternative);

    // convert the C++ `PDFUncertainty` to Rust's `PdfUncertainty`
    PdfUncertainty result;
    result.central = uncertainty.central;
    result.errplus = uncertainty.errplus;
    result.errminus = uncertainty.errminus;
    result.errsymm = uncertainty.errsymm;
    result.scale = uncertainty.scale;
    result.errplus_pdf = uncertainty.errplus_pdf;
    result.errminus_pdf = uncertainty.errminus_pdf;
    result.errsymm_pdf = uncertainty.errsymm_pdf;
    result.err_par = uncertainty.err_par;

    return result;
}

#endif
