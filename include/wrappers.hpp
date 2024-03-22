#ifndef WRAPPERS_HPP
#define WRAPPERS_HPP

#ifdef FAKE_WRAPPERS
#include "fake-lhapdf.hpp"
#else
#include <LHAPDF/LHAPDF.h>
#endif

#include <lhapdf/src/lib.rs.h>
#include <rust/cxx.h>

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#ifdef FAKE_WRAPPERS

inline std::unique_ptr<LHAPDF::PDF> pdf_with_setname_and_member(std::string const&, std::int32_t) {
    return std::unique_ptr<LHAPDF::PDF>();
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_set_and_member(LHAPDF::PDFSet const&, std::int32_t) {
    return std::unique_ptr<LHAPDF::PDF>();
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_setname_and_nmem(std::string const&) {
    return std::unique_ptr<LHAPDF::PDF>();
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_lhaid(std::int32_t) {
    return std::unique_ptr<LHAPDF::PDF>();
}

inline std::unique_ptr<LHAPDF::PDFSet> pdfset_new(std::string const&) {
    return std::unique_ptr<LHAPDF::PDFSet>();
}

inline std::unique_ptr<LHAPDF::PDFSet> pdfset_from_pdf(LHAPDF::PDF const&) {
    return std::unique_ptr<LHAPDF::PDFSet>();
}

inline void lookup_pdf_setname(std::int32_t, std::string&) {}

inline std::int32_t lookup_pdf_memberid(std::int32_t) {
    return 0;
}

inline void get_pdfset_error_type(LHAPDF::PDFSet const&, std::string&) {}

inline PdfUncertainty pdf_uncertainty(
    LHAPDF::PDFSet const&,
    rust::Slice<double const>,
    double,
    bool
) {
    PdfUncertainty result;
    return result;
}

#else

inline std::unique_ptr<LHAPDF::PDF> pdf_with_setname_and_member(
    std::string const& setname,
    std::int32_t member
) {
    return std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(setname, member));
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_set_and_member(
    LHAPDF::PDFSet const& set,
    std::int32_t member
) {
    return pdf_with_setname_and_member(set.name(), member);
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_setname_and_nmem(std::string const& setname_nmem) {
    return std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(setname_nmem));
}

inline std::unique_ptr<LHAPDF::PDF> pdf_with_lhaid(std::int32_t lhaid) {
    return std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(lhaid));
}

inline std::unique_ptr<LHAPDF::PDFSet> pdfset_new(std::string const& setname) {
    return std::unique_ptr<LHAPDF::PDFSet>(new LHAPDF::PDFSet(setname));
}

inline std::unique_ptr<LHAPDF::PDFSet> pdfset_from_pdf(LHAPDF::PDF const& pdf) {
    return std::unique_ptr<LHAPDF::PDFSet>(new LHAPDF::PDFSet(pdf.set()));
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

#endif
