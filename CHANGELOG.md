# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.4] - 15/01/2024

- added `Pdf::flavors` and `Pdf::set_flavors` methods

## [0.2.3] - 04/11/2023

- fixed building error on <https://docs.rs/crate/lhapdf/latest>

## [0.2.2] - 07/07/2022

- explicitly add `-std=c++11` compiler flag to support older compilers

## [0.2.1] - 30/06/2022

- added methods `Pdf::force_positive` and `Pdf::set_force_positive`

## [0.2.0] - 17/05/2022

- added function `Pdf::with_setname_and_nmem`
- changed return type of `PdfSet::uncertainty` to `Result<_>`
- added constant `CL_1_SIGMA`

## [0.1.11] - 19/01/2022

- added methods `Pdf::x_max` and `Pdf::x_min`

## [0.1.10] - 18/03/2021

- added method `PdfSet::error_type`

## [0.1.9] - 01/12/2020

- added method `Pdf::set`

## [0.1.8] - 21/10/2020

- added functions `set_verbosity` and `verbosity`

## [0.1.7] - 16/09/2020

- added a few tests
- added `PdfSet::entry` method

## [0.1.6] - 19/07/2020

- added implementation of `Send` for `Pdf`. See `README.md` for guarantees.

## [0.1.5] - 04/07/2020

## [0.1.4] - 03/07/2020

- added new member `PdfSet::mk_pdfs`

## [0.1.3] - 02/07/2020

- added PDF uncertainty structs and methods
- added function `lookup_pdf`

## [0.1.2] - 31/05/2020

- fixed building on docs.rs

## [0.1.1] - 27/05/2020

- fixed Cargo repository URL and description

## [0.1.0] - 27/05/2020

- first release

[Unreleased]: https://github.com/cschwan/lhapdf/compare/v0.2.4...HEAD
[0.2.3]: https://github.com/cschwan/lhapdf/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/cschwan/lhapdf/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/cschwan/lhapdf/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/cschwan/lhapdf/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/cschwan/lhapdf/compare/v0.1.11...v0.2.0
[0.1.11]: https://github.com/cschwan/lhapdf/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/cschwan/lhapdf/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/cschwan/lhapdf/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/cschwan/lhapdf/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/cschwan/lhapdf/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/cschwan/lhapdf/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/cschwan/lhapdf/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/cschwan/lhapdf/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/cschwan/lhapdf/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cschwan/lhapdf/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cschwan/lhapdf/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cschwan/lhapdf/compare/v0.0.0...v0.1.0
