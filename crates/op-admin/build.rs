use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo"),
    );
    let material = manifest_dir
        .join("../../vendor/material-1.1.0/material.slint")
        .canonicalize()
        .expect("vendor/material-1.1.0/material.slint must exist");

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("material".to_string(), material)]));

    slint_build::compile_with_config("ui/main.slint", config)
        .expect("failed to compile ui/main.slint");

    println!("cargo:rerun-if-changed=ui");
    println!("cargo:rerun-if-changed=../../vendor/material-1.1.0");
}
