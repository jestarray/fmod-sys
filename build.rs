extern crate bindgen;

use std::path::PathBuf;
use std::{env, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("{}", dir);
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("lib").to_str().unwrap()
    );
    // todo: link to debugmode dlls the "L" in fmodL.dll are debug ones that print debug info out
    println!("cargo:rustc-link-lib=fmod");
    //println!("cargo:rustc-link-lib=dylib=fmod");

    // have to copy fmod.lib and name that to fmodstudio.lib in order for this feature to work

    #[cfg(feature = "studio")]
    println!("cargo:rustc-link-lib=fmodstudio");
    //println!("cargo:rustc-link-lib=dylib=fmodstudio");

    let bindings = bindgen::Builder::default()
        .header("core-wrapper.h")
        .generate()
        .expect("Unable to generate core bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-core.rs"))
        .expect("Couldn't write core bindings!");

    let bindings = bindgen::Builder::default()
        .header("studio-wrapper.h")
        .generate()
        .expect("Unable to generate studio bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-studio.rs"))
        .expect("Couldn't write studio bindings!");
}
