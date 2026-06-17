use std::{env, path::PathBuf};

fn main() {
    // -------------------------
    // 1. Build C library with CMake
    // -------------------------
    let dst = cmake::Config::new("deps/c_lib")
        .define("BUILD_SHARED_LIBS", "OFF")
        .no_build_target(true)
        .build();

    // Tell Rust where to find the compiled library
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=c_lib");

    // -------------------------
    // 2. Generate bindings with bindgen
    // -------------------------
    let bindings = bindgen::Builder::default()
        // IMPORTANT: point to your public header(s)
        .header("deps/c_lib/include/c_lib/c_lib.h")
        // If headers depend on include paths:
        .clang_arg(format!("-I{}", "deps/c_lib/include"))
        .generate()
        .expect("bindgen failed");

    // Write to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("could not write bindings");

    // Tell Cargo to rerun if C files or headers change
    println!("cargo:rerun-if-changed=deps/c_lib");

    cxx_build::bridge("src/cpp_lib_wrapper.rs")
        .file("deps/cpp_lib/cpp_lib/user_age_table.cpp")
        .file("deps/cpp_lib/rust_bridge/user_age_table_bridge.cpp")
        .include("deps/cpp_lib/rust_bridge/include")
        .include("deps/cpp_lib/cpp_lib/include")
        .std("c++23")
        .compile("cpp_lib");

    println!("cargo:rerun-if-changed=src/cpp_lib_wrapper.rs");
    println!("cargo:rerun-if-changed=deps/cpp_lib/rust_bridge/include/user_age_table_bridge.h");
    println!("cargo:rerun-if-changed=deps/cpp_lib/rust_bridge/user_age_table_bridge.cpp");
    println!("cargo:rerun-if-changed=deps/cpp_lib/cpp_lib/include/cpp_lib/user_age_table.h");
    println!("cargo:rerun-if-changed=deps/cpp_lib/cpp_lib/user_age_table.cpp");

    // Build protobuf files
    prost_build::compile_protos(
        &[
            "proto/playground/user.proto",
            "proto/playground/objects/car.proto",
        ],
        &["proto/"],
    )
    .expect("Failed to compile proto files");
    println!("cargo:rerun-if-changed=proto/");
}
