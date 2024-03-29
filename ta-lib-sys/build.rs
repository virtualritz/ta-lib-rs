use bindgen::{
    callbacks::{EnumVariantValue, ParseCallbacks},
    Builder,
};
use std::{env, path::PathBuf, process::Command};

#[derive(Debug)]
struct CleanTaNamingCallbacks {}

impl ParseCallbacks for CleanTaNamingCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("TA_") {
            Some(original_item_name.trim_start_matches("TA_").to_string())
        } else {
            None
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        if enum_name.is_some() {
            Some(original_variant_name.trim_start_matches("TA_").to_string())
        } else {
            None
        }
    }
}

const TA_LIB_PATH: &str = "ta-lib-0.4.0";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-lib=static=ta_lib");

    if !cfg!(feature = "use_system_lib") {
        println!(
            "cargo:rustc-link-search=native={}",
            out_path.join("lib").display()
        );

        /*
        let ta_lib = autotools::Config::new(TA_LIB_PATH)
            .enable_static()
            .insource(true)
            .build();

        // Simply link the library without using pkg-config
        println!("cargo:rustc-link-search=native={}", ta_lib.display());
        println!("cargo:rustc-link-lib=static=ta-lib");

        eprintln!("{}", ta_lib.display());
        */

        Command::new("./configure")
            .current_dir(TA_LIB_PATH)
            .arg(format!("--prefix={}", out_path.display()))
            .output()
            .expect("Failed to execute TA C library configure script.");

        Command::new("make")
            .current_dir(TA_LIB_PATH)
            .arg("install")
            .output()
            .expect("Failed to build TA C library.");
    }

    let bindings = Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", out_path.join("include").display()))
        .allowlist_function("TA_.*")
        .allowlist_type("TA_.*")
        .allowlist_var("TA_.*")
        .rustified_enum("TA_.*")
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(CleanTaNamingCallbacks {}))
        .generate()
        .expect("Unable to generate bindings.");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");
}
