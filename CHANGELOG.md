# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.4] - 24/08/2024

- replaced `reqwest` in favor of `ureq` to minimize dependencies

## [0.3.3] - 27/06/2024

- even more fixes to the static linking process to workaround a bug in the
  `pkg-config` crate

## [0.3.2] - 26/06/2024

- more fixes to the linking process

## [0.3.1] - 25/06/2024

- fixed static linking

## [0.3.0] - 23/05/2024

- added feature flag `static` to compile against LHAPDF statically
- added feature flag `managed` to allow disabling the management interface
- changed method `PdfSet::mk_pdfs` to return a `Result` of a `Vec<Pdf>`
- added new method `PdfSet::name`
- renamed `LhapdfError` to `Error`, which now is an enum of `LhapdfException`
  and `General`, which denote exceptions coming from the C++ library and errors
  from the Rust part, respectively
- removed the function `available_pdf_sets`: this cannot be efficiently
  implemented (yet?) when PDF sets are available from multiple repositories
- raised MSRV to 1.70.0

[Unreleased]: https://github.com/cschwan/managed-lhapdf/compare/v0.3.4...HEAD
[0.3.4]: https://github.com/cschwan/managed-lhapdf/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/cschwan/managed-lhapdf/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/cschwan/managed-lhapdf/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/cschwan/managed-lhapdf/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/cschwan/managed-lhapdf/compare/v0.2.4...v0.3.0
