#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![warn(missing_docs)]

//! (Unofficial) Rust wrapper for the [LHAPDF](https://lhapdf.hepforge.org) C++ library.

mod error;
mod ffi;
#[cfg(feature = "managed")]
mod manager;
mod unmanaged;

#[cfg(not(feature = "managed"))]
mod manager {
    pub use super::unmanaged::*;
}

use cxx::{let_cxx_string, CxxVector, UniquePtr};
use std::fmt::{self, Formatter};

pub use error::{Error, Result};
pub use ffi::PdfUncertainty;

/// CL percentage for a Gaussian 1-sigma.
pub const CL_1_SIGMA: f64 = 68.268_949_213_708_58;

/// Convert an LHAID to an LHAPDF set name and member ID.
#[must_use]
pub fn lookup_pdf(lhaid: i32) -> Option<(String, i32)> {
    manager::pdf_name_and_member_via_lhaid(lhaid)
}

/// Convenient way to set the verbosity level.
pub fn set_verbosity(verbosity: i32) {
    manager::set_verbosity(verbosity);
}

/// Convenient way to get the current verbosity level.
#[must_use]
pub fn verbosity() -> i32 {
    manager::verbosity()
}

/// Wrapper to an LHAPDF object of the type `LHAPDF::PDF`.
pub struct Pdf {
    ptr: UniquePtr<ffi::PDF>,
}

impl fmt::Debug for Pdf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: not all PDFs have an LHAID
        f.debug_struct("Pdf")
            .field("lhaid", &self.ptr.lhapdfID())
            .finish()
    }
}

impl Pdf {
    /// Constructor. Create a new PDF with the given `lhaid` ID code.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn with_lhaid(lhaid: i32) -> Result<Self> {
        let Some((setname, member)) = lookup_pdf(lhaid) else {
            return Err(Error::General(format!(
                "did not find PDF with LHAID = {lhaid}"
            )));
        };

        Self::with_setname_and_member(&setname, member)
    }

    /// Constructor. Create a new PDF with the given PDF `setname` and `member` ID.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn with_setname_and_member(setname: &str, member: i32) -> Result<Self> {
        manager::pdf_with_setname_and_member(setname, member).map(|ptr| Self { ptr })
    }

    /// Create a new PDF with the given PDF set name and member ID as a single string.
    ///
    /// The format of the `setname_nmem` string is `<setname>/<nmem>` where `<nmem>` must be
    /// parseable as a positive integer. The `/` character is not permitted in set names due to
    /// clashes with Unix filesystem path syntax.
    ///
    /// If no `/<nmem>` is given, member number 0 will be used.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn with_setname_and_nmem(setname_nmem: &str) -> Result<Self> {
        let (setname, member) = setname_nmem.split_once('/').map_or(
            Ok::<_, Error>((setname_nmem, 0)),
            |(setname, nmem)| {
                Ok((
                    setname,
                    nmem.parse().map_err(|err| {
                        {
                            Error::General(format!(
                                "problem while parsing member index = {nmem}: '{err}'"
                            ))
                        }
                    })?,
                ))
            },
        )?;

        Self::with_setname_and_member(setname, member)
    }

    /// Get the PDF `x * f(x)` value at `x` and `q2` for the given PDG ID.
    ///
    /// # Panics
    ///
    /// If the value of either `x` or `q2` is not within proper boundaries this method will panic.
    #[must_use]
    pub fn xfx_q2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.ptr.xfxQ2(id, x, q2).unwrap()
    }

    /// Value of of the strong coupling at `q2` used by this PDF.
    ///
    /// # Panics
    ///
    /// If the value of `q2` is not within proper boundaries this method will panic.
    #[must_use]
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.ptr.alphasQ2(q2).unwrap()
    }

    /// Get the info class that actually stores and handles the metadata.
    #[must_use]
    pub fn set(&self) -> PdfSet {
        let_cxx_string!(setname = "");
        ffi::pdf_setname(&self.ptr, setname.as_mut());
        // UNWRAP: if `setname` contains any non-UTF8 bytes there's an error somewhere else
        let setname = setname.to_str().unwrap_or_else(|_| unreachable!());

        // UNWRAP: if a `PDF` doesn't have a `PDFSet` there's a bug somewhere
        PdfSet::new(setname).unwrap_or_else(|_| unreachable!())
    }

    /// Minimum valid x value for this PDF.
    #[must_use]
    pub fn x_min(&mut self) -> f64 {
        self.ptr.pin_mut().xMin()
    }

    /// Maximum valid x value for this PDF.
    #[must_use]
    pub fn x_max(&mut self) -> f64 {
        self.ptr.pin_mut().xMax()
    }

    /// Set whether the PDF will only return positive (definite) values or not.
    pub fn set_force_positive(&mut self, mode: i32) {
        self.ptr.pin_mut().setForcePositive(mode);
    }

    /// Check whether the PDF is set to only return positive (definite) values or not.
    ///
    /// This is to avoid overshooting in to negative values when interpolating/extrapolating PDFs
    /// that sharply decrease towards zero. 0 = unforced, 1 = forced positive, 2 = forced positive
    /// definite (>= 1e-10).
    #[must_use]
    pub fn force_positive(&mut self) -> i32 {
        self.ptr.pin_mut().forcePositive()
    }

    /// List of flavours defined by this [`Pdf`] set.
    #[must_use]
    pub fn flavors(&self) -> Vec<i32> {
        self.ptr.flavors().iter().copied().collect()
    }

    /// Manually set/override the list of flavours defined by this [`Pdf`] set.
    pub fn set_flavors(&mut self, flavors: &[i32]) {
        let mut vector = CxxVector::new();

        flavors
            .iter()
            .for_each(|&flavor| vector.pin_mut().push(flavor));

        self.ptr.pin_mut().setFlavors(&vector);
    }
}

unsafe impl Send for Pdf {}
unsafe impl Sync for Pdf {}

/// Class for PDF set metadata and manipulation.
pub struct PdfSet {
    ptr: UniquePtr<ffi::PDFSet>,
}

impl fmt::Debug for PdfSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: a PDF set may not have an LHAPDF ID
        f.debug_struct("PdfSet")
            .field("lhaid", &self.ptr.lhapdfID())
            .finish()
    }
}

impl PdfSet {
    /// Constructor from a set name.
    ///
    /// # Errors
    ///
    /// If the PDF set with the specified name was not found an error is returned.
    pub fn new(setname: &str) -> Result<Self> {
        manager::pdfset_new(setname).map(|ptr| Self { ptr })
    }

    /// Retrieve a metadata string by key name.
    #[must_use]
    pub fn entry(&self, key: &str) -> Option<String> {
        let_cxx_string!(cxx_key = key);

        if self.ptr.has_key(&cxx_key) {
            Some(self.ptr.get_entry(&cxx_key).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    /// Get the type of PDF errors in this set (replicas, symmhessian, hessian, custom, etc.).
    #[must_use]
    pub fn error_type(&self) -> String {
        let_cxx_string!(string = "");

        ffi::get_pdfset_error_type(&self.ptr, string.as_mut());
        string.to_string_lossy().into_owned()
    }

    /// Make all the PDFs in this set.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn mk_pdfs(&self) -> Result<Vec<Pdf>> {
        let setname = self.name();

        // UNWRAP: if we can't convert a `usize` to an `i32`, then we probably got too many members
        // indicating a bug somewher
        (0..i32::try_from(self.ptr.size()).unwrap_or_else(|_| unreachable!()))
            .map(|member| Pdf::with_setname_and_member(&setname, member))
            .collect::<Result<Vec<_>>>()
    }

    /// PDF set name.
    #[must_use]
    pub fn name(&self) -> String {
        let_cxx_string!(setname = "");
        ffi::pdfset_setname(&self.ptr, setname.as_mut());
        // UNWRAP: if `setname` contains any non-UTF8 bytes there's an error somewhere else
        let setname = setname.to_str().unwrap_or_else(|_| unreachable!());

        setname.to_owned()
    }

    /// Calculate central value and error from vector values with appropriate formulae for this
    /// set.
    ///
    /// Warning: The values vector corresponds to the members of this PDF set and must be ordered
    /// accordingly.
    ///
    /// In the Hessian approach, the central value is the best-fit "values\[0\]" and the uncertainty
    /// is given by either the symmetric or asymmetric formula using eigenvector PDF sets.
    ///
    /// If the PDF set is given in the form of replicas, by default, the central value is given by
    /// the mean and is not necessarily "values\[0]\" for quantities with a non-linear dependence on
    /// PDFs, while the uncertainty is given by the standard deviation.
    ///
    /// The argument `cl` is used to rescale uncertainties to a particular confidence level (in
    /// percent); a negative number will rescale to the default CL for this set. The default value
    /// in LHAPDF is `100*erf(1/sqrt(2))=68.268949213709`, corresponding to 1-sigma uncertainties.
    ///
    /// If the PDF set is given in the form of replicas, then the argument `alternative` equal to
    /// `true` (default in LHAPDF: `false`) will construct a confidence interval from the
    /// probability distribution of replicas, with the central value given by the median.
    ///
    /// For a combined set, a breakdown of the separate PDF and parameter variation uncertainties
    /// is available. The parameter variation uncertainties are computed from the last `2*n`
    /// members of the set, with `n` the number of parameters.
    ///
    /// # Errors
    ///
    /// TODO
    pub fn uncertainty(
        &self,
        values: &[f64],
        cl: f64,
        alternative: bool,
    ) -> Result<PdfUncertainty> {
        Ok(ffi::pdf_uncertainty(&self.ptr, values, cl, alternative)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_verbosity() {
        super::set_verbosity(0);
        assert_eq!(verbosity(), 0);
    }

    #[test]
    fn check_lookup_pdf() {
        assert!(matches!(lookup_pdf(324900), Some((name, member))
            if (name == "NNPDF31_nlo_as_0118_luxqed") && (member == 0)));
        assert!(matches!(lookup_pdf(324901), Some((name, member))
            if (name == "NNPDF31_nlo_as_0118_luxqed") && (member == 1)));
        assert!(matches!(lookup_pdf(-1), None));
    }

    #[test]
    fn debug_pdf() -> Result<()> {
        let pdf = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 0)?;

        assert_eq!(format!("{:?}", pdf), "Pdf { lhaid: 324900 }");

        Ok(())
    }

    #[test]
    fn check_pdf() -> Result<()> {
        let mut pdf_0 = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 0)?;
        let mut pdf_1 = Pdf::with_lhaid(324900)?;

        let value_0 = pdf_0.xfx_q2(2, 0.5, 90.0 * 90.0);
        let value_1 = pdf_1.xfx_q2(2, 0.5, 90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        let value_0 = pdf_0.alphas_q2(90.0 * 90.0);
        let value_1 = pdf_1.alphas_q2(90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        assert_eq!(
            Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 10000)
                .unwrap_err()
                .to_string(),
            "PDF NNPDF31_nlo_as_0118_luxqed/10000 is out of the member range of set NNPDF31_nlo_as_0118_luxqed"
        );

        assert_eq!(
            Pdf::with_lhaid(0).unwrap_err().to_string(),
            "did not find PDF with LHAID = 0"
        );

        assert_eq!(pdf_0.x_min(), 1e-9);
        assert_eq!(pdf_0.x_max(), 1.0);
        assert_eq!(pdf_1.x_min(), 1e-9);
        assert_eq!(pdf_1.x_max(), 1.0);

        Ok(())
    }

    #[test]
    fn check_setname_and_nmem() -> Result<()> {
        let pdf_0 = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 1)?;
        let pdf_1 = Pdf::with_setname_and_nmem("NNPDF31_nlo_as_0118_luxqed/1")?;

        let value_0 = pdf_0.xfx_q2(2, 0.5, 90.0 * 90.0);
        let value_1 = pdf_1.xfx_q2(2, 0.5, 90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        let value_0 = pdf_0.alphas_q2(90.0 * 90.0);
        let value_1 = pdf_1.alphas_q2(90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        assert_eq!(
            Pdf::with_setname_and_nmem("foobar/0")
                .unwrap_err()
                .to_string(),
            "Info file not found for PDF set 'foobar'"
        );

        assert_eq!(
            Pdf::with_setname_and_nmem("NNPDF31_nlo_as_0118_luxqed/x")
                .unwrap_err()
                .to_string(),
            "problem while parsing member index = x: 'invalid digit found in string'"
        );

        Ok(())
    }

    #[test]
    fn check_pdf_set() -> Result<()> {
        let pdf_set = PdfSet::new("NNPDF31_nlo_as_0118_luxqed")?;

        assert!(matches!(pdf_set.entry("Particle"), Some(value) if value == "2212"));
        assert!(matches!(pdf_set.entry("Flavors"), Some(value)
            if value == "[-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 22]"));
        assert_eq!(pdf_set.entry("idontexist"), None);

        assert_eq!(pdf_set.error_type(), "replicas");
        assert_eq!(pdf_set.name(), "NNPDF31_nlo_as_0118_luxqed");

        assert_eq!(
            PdfSet::new("IDontExist").unwrap_err().to_string(),
            "Info file not found for PDF set 'IDontExist'"
        );

        assert_eq!(pdf_set.mk_pdfs().unwrap().len(), 101);

        let uncertainty = pdf_set.uncertainty(&[0.0; 101], 68.268949213709, false)?;

        assert_eq!(uncertainty.central, 0.0);
        assert_eq!(uncertainty.central, 0.0);
        assert_eq!(uncertainty.errplus, 0.0);
        assert_eq!(uncertainty.errminus, 0.0);
        assert_eq!(uncertainty.errsymm, 0.0);
        //assert_eq!(uncertainty.scale, 1.0);
        assert_eq!(uncertainty.errplus_pdf, 0.0);
        assert_eq!(uncertainty.errminus_pdf, 0.0);
        assert_eq!(uncertainty.errsymm_pdf, 0.0);
        assert_eq!(uncertainty.err_par, 0.0);

        Ok(())
    }

    #[test]
    fn debug_pdf_set() -> Result<()> {
        let pdf_set = PdfSet::new("NNPDF31_nlo_as_0118_luxqed")?;

        assert_eq!(format!("{:?}", pdf_set), "PdfSet { lhaid: 324900 }");

        Ok(())
    }

    #[test]
    fn check_pdf_pdfset() -> Result<()> {
        let pdf_set0 = PdfSet::new("NNPDF31_nlo_as_0118_luxqed")?;
        let pdf_set1 = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 0)?.set();

        assert_eq!(pdf_set0.entry("Particle"), pdf_set1.entry("Particle"));
        assert_eq!(pdf_set0.entry("NumMembers"), pdf_set1.entry("NumMembers"));

        Ok(())
    }

    #[test]
    fn force_positive() -> Result<()> {
        let mut pdf = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 1)?;

        assert_eq!(pdf.force_positive(), 0);

        pdf.set_force_positive(1);
        assert_eq!(pdf.force_positive(), 1);

        Ok(())
    }

    #[test]
    fn set_flavors() {
        let mut pdf = Pdf::with_setname_and_member("NNPDF31_nlo_as_0118_luxqed", 0).unwrap();

        assert_eq!(pdf.flavors(), &[-5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 21, 22]);

        pdf.set_flavors(&[-5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 21]);

        assert_eq!(pdf.flavors(), &[-5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 21]);
    }

    #[test]
    fn download_pdf_set() {
        let _ = Pdf::with_setname_and_member("CT10", 0).unwrap();
    }
}
