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

    let bindings_core = "bindings-core.rs";
    #[allow(unused_variables)]
    let bindings_studio = "bindings-studio.rs";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    #[cfg(feature = "bindgen")]
    {
        use bindgen;
        let bindings = bindgen::Builder::default()
            .header("core-wrapper.h")
            .rustified_enum(".*")
            .derive_default(true)
            .generate()
            .expect("Unable to generate core bindings");

        bindings
            .write_to_file(out_dir.join(bindings_core))
            .expect("Couldn't write core bindings!");

        #[cfg(feature = "studio")]
        {
            let bindings = bindgen::Builder::default()
                .header("studio-wrapper.h")
                .rustified_enum(".*")
                .derive_default(true)
                .generate()
                .expect("Unable to generate studio bindings");

            bindings
                .write_to_file(out_dir.join(bindings_studio))
                .expect("Couldn't write studio bindings!");
        }
    }

    #[cfg(not(feature = "bindgen"))]
    {
        // copy pre-generated bindings to out dir
        let gen_dir = PathBuf::from("./pre-gen/");
        let core_bindings = gen_dir.join(bindings_core);
        std::fs::copy(core_bindings, out_dir.join(bindings_core)).unwrap();

        #[cfg(feature = "studio")]
        {
            let studio_bindings = gen_dir.join(bindings_studio);
            std::fs::copy(studio_bindings, out_dir.join(bindings_studio)).unwrap();
        }
    }
}
