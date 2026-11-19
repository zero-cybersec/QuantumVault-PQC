use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=QUANTUMVAULT_CRYPTO_LIB_DIR");
    println!("cargo:rerun-if-env-changed=QUANTUMVAULT_CRYPTO_LIB");

    if let Ok(lib_dir) = env::var("QUANTUMVAULT_CRYPTO_LIB_DIR") {
        println!("cargo:rustc-link-search=native={lib_dir}");

        let lib_name = env::var("QUANTUMVAULT_CRYPTO_LIB")
            .unwrap_or_else(|_| "quantumvault_crypto".to_string());

        println!("cargo:rustc-link-lib=static={lib_name}");
    }

    println!("cargo:warning=QuantumVault native crypto linking configured");
}
