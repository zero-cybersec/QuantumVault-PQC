use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run(command: &mut Command) {
    let status = command
        .status()
        .expect("Failed to execute native build command");

    assert!(status.success(), "Native build command failed");
}

fn main() {
    let source = "native/crypto/quantumvault_crypto.c";
    let header = "native/crypto/quantumvault_crypto.h";

    println!("cargo:rerun-if-changed={source}");
    println!("cargo:rerun-if-changed={header}");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));

    let object_file = out_dir.join("quantumvault_crypto.o");
    let library_file = out_dir.join("libquantumvault_crypto.a");

    run(Command::new("cc")
        .args([
            "-std=c11",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-I",
            "native/crypto",
            "-c",
            source,
            "-o",
        ])
        .arg(&object_file));

    run(Command::new("ar")
        .arg("crs")
        .arg(&library_file)
        .arg(&object_file));

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=quantumvault_crypto");

    println!("cargo:warning=QuantumVault C crypto library compiled and linked");
}
