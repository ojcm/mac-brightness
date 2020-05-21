extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Rebuild if the bindings change
    println!("cargo:rerun-if-changed=bindings.h");

    // Produce bindings
    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link the OSX frameworks via cargo
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=ApplicationServices");
}