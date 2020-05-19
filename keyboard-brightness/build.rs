fn main() {
    // Link the OSX frameworks via cargo
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=ApplicationServices");
}