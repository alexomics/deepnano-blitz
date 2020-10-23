use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("python")
        .arg(".github/workflows/get_mkl_lib.py")
        .arg(&out_dir)
        .status()
        .unwrap();

    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static-nobundle=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_sequential");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_core");
}
