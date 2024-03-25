[![Rust](https://github.com/cschwan/managed-lhapdf/actions/workflows/rust.yml/badge.svg)](https://github.com/cschwan/managed-lhapdf/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/cschwan/managed-lhapdf/graph/badge.svg?token=H8Q8JHXY1K)](https://codecov.io/gh/cschwan/managed-lhapdf)
[![Documentation](https://docs.rs/lhapdf/badge.svg)](https://docs.rs/managed-lhapdf)
[![crates.io](https://img.shields.io/crates/v/managed-lhapdf.svg)](https://crates.io/crates/managed-lhapdf)
![MSRV](https://img.shields.io/badge/Rust-1.70+-lightgray.svg)]

# Description

(Unofficial) Rust bindings for the [LHAPDF](https://lhapdf.hepforge.org) C++
library, with automatic management functions. This is the successor of
<https://github.com/cschwan/lhapdf>.

# (Un)safeness

The struct `Pdf` implements `Send` and `Sync`, which is only safe as long as
the corresponding member functions in LHAPDF are truly thread safe. The
following versions are known not to be thread safe:

- 6.4.x, see [LHAPDF merge request #27](https://gitlab.com/hepcedar/lhapdf/-/merge_requests/27)
- 6.3.x, see [LHAPDF issue #2](https://gitlab.com/hepcedar/lhapdf/-/issues/2)
