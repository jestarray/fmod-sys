use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    // if running into issues with windows not being able to find the dll, copy fmodL_vc.lib and name it to: fmodL.lib
    //https://www.fmod.com/docs/2.03/api/platforms-win.html#libraries

    #[cfg(target_os = "windows")]
    {
        //WINDOWS
        #[cfg(not(feature = "debug"))]
        println!("cargo:rustc-link-lib=fmod_vc");

        #[cfg(feature = "debug")]
        println!("cargo:rustc-link-lib=fmodL_vc");

        #[cfg(all(feature = "studio", feature = "debug"))]
        println!("cargo:rustc-link-lib=fmodstudioL_vc");

        #[cfg(all(feature = "studio", not(feature = "debug")))]
        println!("cargo:rustc-link-lib=fmodstudio_vc");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // copy the lib files to a lib folder in target
        // note that you will have to configure rust rpath to $ORIGIN/lib . See README.md
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        let lib_folder = Path::new(&dir).join("lib_x64_linux");

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        let lib_folder = Path::new(&dir).join("lib_arm64_linux");

        #[cfg(target_os = "macos")]
        let lib_folder = Path::new(&dir).join("lib_macos");

        let dir = lib_folder.to_str().unwrap();
        println!("cargo:rustc-link-search={}", dir);

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        // This usually gets you to target/debug or target/release
        let target_dir = out_dir.ancestors().nth(3).unwrap();
        let target_lib_dir = target_dir.join("lib");

        // Create the target/debug/lib folder if it doesn't exist
        fs::create_dir_all(&target_lib_dir).ok();

        // Copy the library there so 'cargo run' works
        let _ = copy_dir_all(dir, target_lib_dir);

        #[cfg(not(feature = "debug"))]
        println!("cargo:rustc-link-lib=fmod");

        #[cfg(feature = "debug")]
        println!("cargo:rustc-link-lib=fmodL");

        #[cfg(all(feature = "studio", not(feature = "debug")))]
        println!("cargo:rustc-link-lib=fmodstudio");

        #[cfg(all(feature = "studio", feature = "debug"))]
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

#[allow(unused)]
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.as_ref().join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}
