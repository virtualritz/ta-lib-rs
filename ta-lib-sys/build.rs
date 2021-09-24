extern crate bindgen;

use std::{env, path::PathBuf, process::Command};

fn main() {
    // Tell cargo to tell rustc to link the system ta_lib
    // shared library.

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    eprintln!("{}", out_dir.to_str().unwrap());

    Command::new("./configure")
        .current_dir("ta-lib")
        .arg(format!("--prefix={}", out_dir.to_str().unwrap()))
        .output()
        .expect("Failed to execute TA C library configure script");

    Command::new("make")
        .current_dir("ta-lib")
        .arg("install")
        .output()
        .expect("Failed to build TA C library");

    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=ta_lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", out_dir.join("include").to_str().unwrap()))
        // Generate rustified enums
        .rustified_enum(".*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
