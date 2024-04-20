pub use ffi::*;

// ALLOW: as soon as the `cxx` offers a `#![cxx::bridge]` we can get rid of the `mod ffi`
#[allow(clippy::module_inception)]
#[cxx::bridge]
mod ffi {
    // The type `PdfUncertainty` must be separate from the one defined in the C++ namespace LHAPDF
    // because it differs (at least) from LHAPDF 6.4.x to 6.5.x

    /// Structure for storage of uncertainty info calculated over a PDF error set.
    struct PdfUncertainty {
        /// The central value.
        pub central: f64,
        /// The unsymmetric error in positive direction.
        pub errplus: f64,
        /// The unsymmetric error in negative direction.
        pub errminus: f64,
        /// The symmetric error.
        pub errsymm: f64,
        /// The scale factor needed to convert between the PDF set's default confidence level and
        /// the requested confidence level.
        pub scale: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errplus_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errminus_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errsymm_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub err_par: f64,
    }

    #[namespace = "LHAPDF"]
    unsafe extern "C++" {
        include!("managed-lhapdf/include/lhapdf.hpp");

        fn setVerbosity(verbosity: i32);
        fn verbosity() -> i32;

        type PDF;

        fn alphasQ2(self: &PDF, q2: f64) -> Result<f64>;
        fn xfxQ2(self: &PDF, id: i32, x: f64, q2: f64) -> Result<f64>;
        fn lhapdfID(self: &PDF) -> i32;
        fn xMin(self: Pin<&mut PDF>) -> f64;
        fn xMax(self: Pin<&mut PDF>) -> f64;
        fn setFlavors(self: Pin<&mut PDF>, flavors: &CxxVector<i32>);
        fn setForcePositive(self: Pin<&mut PDF>, mode: i32);
        fn flavors<'a>(self: &'a PDF) -> &'a CxxVector<i32>;
        fn forcePositive(self: &PDF) -> i32;

        type PDFSet;

        fn has_key(self: &PDFSet, key: &CxxString) -> bool;
        fn get_entry<'a>(self: &PDFSet, key: &'a CxxString) -> &'a CxxString;
        fn size(self: &PDFSet) -> usize;
        fn lhapdfID(self: &PDFSet) -> i32;
    }

    unsafe extern "C++" {
        include!("managed-lhapdf/include/wrappers.hpp");

        fn pdf_setname(pdf: &PDF, setname: Pin<&mut CxxString>);
        fn pdf_with_setname_and_member(setname: &CxxString, member: i32) -> Result<UniquePtr<PDF>>;
        fn pdfset_new(setname: &CxxString) -> Result<UniquePtr<PDFSet>>;
        fn pdfset_setname(pdf: &PDFSet, setname: Pin<&mut CxxString>);

        #[cfg(feature = "managed")]
        fn empty_lhaindex();

        fn lookup_pdf_setname(lhaid: i32, setname: Pin<&mut CxxString>);
        fn lookup_pdf_memberid(lhaid: i32) -> i32;
        fn get_pdfset_error_type(set: &PDFSet, setname: Pin<&mut CxxString>);

        fn pdf_uncertainty(
            pdfset: &PDFSet,
            values: &[f64],
            cl: f64,
            alternative: bool,
        ) -> Result<PdfUncertainty>;
    }
}
