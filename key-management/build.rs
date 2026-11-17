fn main() {
    println!("cargo:rerun-if-changed=src/oqs_bridge.c");

    cc::Build::new()
        .file("src/oqs_bridge.c")
        .include("/usr/local/include")
        .compile("qv_oqs_bridge");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=oqs");
}
