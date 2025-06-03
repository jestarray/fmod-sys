extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("{}", dir);
    /*   println!(
           "cargo:rustc-link-search={}",
           Path::new(&dir).join("lib").to_str().unwrap()
       );
    */
    //println!("cargo:rustc-link-search=/mnt/z/project_s");
    //dbg!(dir);
    // copy fmodL_vc.lib and name it to: fmodL.lib
    {
        //WINDOWS
        #[cfg(all(not(feature = "debug"), target_os = "windows"))]
        println!("cargo:rustc-link-lib=fmod");

        #[cfg(all(feature = "debug", target_os = "windows"))]
        println!("cargo:rustc-link-lib=fmodL");

        //https://fmod.com/resources/documentation-api?version=2.1&page=platforms-windows.html#libraries
        //_vc are release binaries

        // have to copy fmodstudio_vc.lib and name that to fmodstudioL.lib in order for this feature to work
        #[cfg(all(feature = "studio", feature = "debug", target_os = "windows"))]
        println!("cargo:rustc-link-lib=fmodstudioL");

        #[cfg(all(feature = "studio", not(feature = "debug"), target_os = "windows"))]
        println!("cargo:rustc-link-lib=fmodstudio");
    }

    {
        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-search={}", dir);
        //LINUX
        #[cfg(all(not(feature = "debug"), target_os = "linux"))]
        println!("cargo:rustc-link-lib=fmod");

        #[cfg(all(feature = "debug", target_os = "linux"))]
        println!("cargo:rustc-link-lib=fmodL");

        #[cfg(all(feature = "studio", not(feature = "debug"), target_os = "linux"))]
        println!("cargo:rustc-link-lib=fmodstudio");

        #[cfg(all(feature = "studio", feature = "debug", target_os = "linux"))]
        println!("cargo:rustc-link-lib=fmodstudioL");
    }

    let bindings = bindgen::Builder::default()
        .header("core-wrapper.h")
        .rustified_enum(".*")
        .derive_default(true)
        .generate()
        .expect("Unable to generate core bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-core.rs"))
        .expect("Couldn't write core bindings!");

    let bindings = bindgen::Builder::default()
        .header("studio-wrapper.h")
        .rustified_enum(".*")
        .derive_default(true)
        .generate()
        .expect("Unable to generate studio bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-studio.rs"))
        .expect("Couldn't write studio bindings!");
}
