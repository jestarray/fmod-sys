extern crate bindgen;

use std::path::PathBuf;
use std::{env, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("{}", dir);
    /* println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("lib").to_str().unwrap()
    ); */

    // copy fmodL_vc.lib and name it to: fmodL.lib
    #[cfg(not(feature = "debug"))]
    println!("cargo:rustc-link-lib=fmod");

    #[cfg(feature = "debug")]
    println!("cargo:rustc-link-lib=fmodL");

    // have to copy fmodstudio_vc.lib and name that to fmodstudioL.lib in order for this feature to work
    #[cfg(all(feature = "studio", feature = "debug"))]
    println!("cargo:rustc-link-lib=fmodstudioL");

    #[cfg(all(feature = "studio", not(feature = "debug")))]
    println!("cargo:rustc-link-lib=fmodstudio");

    let bindings = bindgen::Builder::default()
        .header("core-wrapper.h")
        .rustified_enum("*")
        .generate()
        .expect("Unable to generate core bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-core.rs"))
        .expect("Couldn't write core bindings!");

    let bindings = bindgen::Builder::default()
        .header("studio-wrapper.h")
        .rustified_enum("*")
        .generate()
        .expect("Unable to generate studio bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-studio.rs"))
        .expect("Couldn't write studio bindings!");
}
