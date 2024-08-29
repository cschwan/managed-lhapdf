[![Rust](https://github.com/cschwan/managed-lhapdf/actions/workflows/rust.yml/badge.svg)](https://github.com/cschwan/managed-lhapdf/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/cschwan/managed-lhapdf/graph/badge.svg?token=H8Q8JHXY1K)](https://codecov.io/gh/cschwan/managed-lhapdf)
[![Documentation](https://docs.rs/lhapdf/badge.svg)](https://docs.rs/managed-lhapdf)
[![crates.io](https://img.shields.io/crates/v/managed-lhapdf.svg)](https://crates.io/crates/managed-lhapdf)
![MSRV](https://img.shields.io/badge/Rust-1.70+-lightgray.svg)

# Description

(Unofficial) Rust bindings for the [LHAPDF](https://lhapdf.hepforge.org) C++
library, with automatic management functions. This is the successor of
<https://github.com/cschwan/lhapdf>.

# Rust feature flags

- `managed`: this feature flag enables the automatic downloading of PDFs. See
  the section below on how to configure its behavior. If you would like to
  disable this feature, specify `no-default-features = true` when depending on
  `managed-lhapdf`.
- `static`: when enabled, the LHAPDF library will be linked statically. This
  allows to redistribute built binaries that run on systems where LHAPDF isn't
  installed.

# Automatic PDF management

If enabled, this crate automatically downloads the required PDF sets. The
behavior can be controlled with the configuration file `managed-lhapdf.toml` in
the [user's data directory](https://docs.rs/dirs/latest/dirs/fn.data_dir.html).
This file is automatically created if it does not exist. The configuration
should look similar to the following one:

```toml
# these paths are scanned for PDF sets, in the given order, and multiple paths
# can be given as # strings seperated by commas. This crate will *not* write into
# any of these directories
lhapdf_data_path_read = []
# if the following path is an empty string, nothing will be automatically
# downloaded. If the path is given, however, this crate will download PDFs sets
# and place them in here that are not found in the previous directories
lhapdf_data_path_write = "/home/alice/.local/share/LHAPDF"
# URL for the pdfsets.index file, which is used to translate LHAIDs to PDF set
# names
pdfsets_index_url = "https://lhapdfsets.web.cern.ch/current/pdfsets.index"
# URLs from which PDF sets are downloaded, in the given order. If a set is not
# found for the first URL, the second URL (and so on) will be tried
pdfset_urls = [
    "https://lhapdfsets.web.cern.ch/current/",
    "https://data.nnpdf.science/pdfs/",
    "https://data.nnpdf.science/pineappl/pdfs/",
]
```

# (Un)safeness

The struct `Pdf` implements `Send` and `Sync`, which is only safe as long as
the corresponding member functions in LHAPDF are truly thread safe. The
following versions are known not to be thread safe:

- 6.4.x, see [LHAPDF merge request #27](https://gitlab.com/hepcedar/lhapdf/-/merge_requests/27)
- 6.3.x, see [LHAPDF issue #2](https://gitlab.com/hepcedar/lhapdf/-/issues/2)
