#[cfg(not(feature = "docs-only"))]
fn main() {
    // get LHAPDF's include directories
    let lhapdf = pkg_config::Config::new()
        .atleast_version("6")
        .cargo_metadata(false)
        .statik(cfg!(feature = "static"))
        .probe("lhapdf")
        .unwrap();

    let mut build = cxx_build::bridge("src/ffi.rs");

    for include_path in lhapdf.include_paths {
        build.include(include_path);
    }

    build
        .flag_if_supported("-std=c++11")
        .compile("lhapdf-rust-cxx-bridge");

    println!("cargo:rerun-if-changed=include/wrappers.hpp");
    println!("cargo:rerun-if-changed=src/ffi.rs");

    // emit linking information AFTER compiling the bridge
    pkg_config::Config::new()
        .atleast_version("6")
        .statik(cfg!(feature = "static"))
        .probe("lhapdf")
        .unwrap();
}

#[cfg(feature = "docs-only")]
fn main() {
    cxx_build::bridge("src/ffi.rs")
        .define("FAKE_WRAPPERS", "1")
        .compile("managed-lhapdf-rust-cxx-bridge");

    println!("cargo:rerun-if-changed=include/fake-lhapdf.hpp");
    println!("cargo:rerun-if-changed=include/wrappers.hpp");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
