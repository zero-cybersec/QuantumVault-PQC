fn main() {
    println!("cargo:rustc-link-search=native=/tmp");
    println!("cargo:rustc-link-lib=dylib=quantumvault_crypto");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=oqs");
    println!("cargo:rerun-if-changed=build.rs");
}
