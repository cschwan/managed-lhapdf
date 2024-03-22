[![Rust](https://github.com/cschwan/lhapdf/workflows/Rust/badge.svg)](https://github.com/cschwan/lhapdf/actions?query=workflow%3ARust)
[![codecov](https://codecov.io/gh/cschwan/lhapdf/branch/cxx/graph/badge.svg)](https://codecov.io/gh/cschwan/lhapdf)
[![Documentation](https://docs.rs/lhapdf/badge.svg)](https://docs.rs/lhapdf)
[![crates.io](https://img.shields.io/crates/v/lhapdf.svg)](https://crates.io/crates/lhapdf)

# Description

(Unofficial) Rust bindings for the [LHAPDF](https://lhapdf.hepforge.org) C++
library

# (Un)safeness

The struct `Pdf` implements `Send` and `Sync`, which is only safe as long as
the corresponding member functions in LHAPDF are truly thread safe. The
following versions are known not to be thread safe:

- 6.4.x, see [LHAPDF merge request #27](https://gitlab.com/hepcedar/lhapdf/-/merge_requests/27)
- 6.3.x, see [LHAPDF issue #2](https://gitlab.com/hepcedar/lhapdf/-/issues/2)
