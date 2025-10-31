use std::env;
use std::fs;
use std::path::PathBuf;


const LINKER_SCRIPT: &str = "memory.x";


fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy linker file to build directory
    fs::copy(LINKER_SCRIPT, out_dir.join(LINKER_SCRIPT)).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
}
