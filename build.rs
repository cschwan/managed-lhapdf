#[cfg(not(feature = "docs-only"))]
fn main() {
    let lhapdf = pkg_config::Config::new()
        .atleast_version("6")
        .probe("lhapdf")
        .unwrap();

    let mut build = cxx_build::bridge("src/ffi.rs");

    for include_path in lhapdf.include_paths {
        build.include(include_path);
    }

    build
        .flag_if_supported("-std=c++11")
        .compile("lhapdf-rust-cxx-bridge");

    for lib_path in lhapdf.link_paths {
        println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    }

    let link_modifier = if cfg!(feature = "static") {
        "static="
    } else {
        ""
    };

    for lib in lhapdf.libs {
        println!("cargo:rustc-link-lib={link_modifier}{lib}");
    }

    println!("cargo:rerun-if-changed=include/wrappers.hpp");
    println!("cargo:rerun-if-changed=src/ffi.rs");
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
