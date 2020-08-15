extern crate os_info;
extern crate reqwest;
use bzip2::read::BzDecoder;
use tar::Archive;
use std::env;
use std::fs::File;
use std::io;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let arch = match os_info::get().os_type() {
        os_info::Type::Macos => "osx-64",
        os_info::Type::Windows => "win-64",
        _ => "linux-64",
    };

    let url = format!("https://anaconda.org/intel/mkl-static/2020.0/download/{}/mkl-static-2020.0-intel_166.tar.bz2", arch);
    let archive = format!("{}/mkl-static.tar.bz2", out_dir);

    let mut out = File::create(&archive).expect("failed to create mkl archive");
    let mut resp = reqwest::blocking::get(&url).expect("request failed");
    io::copy(&mut resp, &mut out).expect("failed to write archive");

    let archive_file = File::open(&archive).expect("Could not open archive");
    let tar = BzDecoder::new(archive_file);
    let mut _archive = Archive::new(tar);
    _archive.unpack(out_dir).expect("uh-oh");

    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static-nobundle=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_sequential");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_core");
}
