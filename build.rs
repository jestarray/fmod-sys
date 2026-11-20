use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // if running into issues with windows not being able to find the dll, copy fmodL_vc.lib and name it to: fmodL.lib
    //https://www.fmod.com/docs/2.03/api/platforms-win.html#libraries

    if target_os == "windows" {
        //WINDOWS
        #[cfg(not(feature = "debug"))]
        println!("cargo:rustc-link-lib=fmod_vc");

        #[cfg(feature = "debug")]
        println!("cargo:rustc-link-lib=fmodL_vc");

        #[cfg(all(feature = "studio", feature = "debug"))]
        println!("cargo:rustc-link-lib=fmodstudioL_vc");

        #[cfg(all(feature = "studio", not(feature = "debug")))]
        println!("cargo:rustc-link-lib=fmodstudio_vc");
    } else {
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        // copy the lib files to a lib folder in target
        // note that you will have to configure rust rpath to $ORIGIN/lib . See README.md
        println!("cargo:rerun-if-env-changed=FMOD_ANDROID_LIB_DIR");
        let lib_folder = if target_os == "android" {
            PathBuf::from(env::var("FMOD_ANDROID_LIB_DIR")
                .expect("Set FMOD_ANDROID_LIB_DIR to Android SDK libraries; Linux ARM64 libraries do not work on Android"))
        } else if target_os == "linux" && target_arch == "x86_64" {
            Path::new(&dir).join("lib_x64_linux")
        } else if target_os == "linux" && target_arch == "aarch64" {
            Path::new(&dir).join("lib_arm64_linux")
        } else if target_os == "macos" {
            Path::new(&dir).join("lib_macos")

        } else {
            panic!(
                "cargo:warning=Unsupported target: {}-{}",
                target_arch, target_os
            );
        };
        let dir = lib_folder.to_str().unwrap();

        println!("cargo:rustc-link-search={}", dir);

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        // OUT_DIR is <target>/<profile>/build/<pkg>-<hash>/out or
        // <target>/<profile>/build/<pkg>/<hash>/out depending on the cargo version,
        // so anchor on the `build` component instead of counting ancestors.
        let target_dir = out_dir
            .ancestors()
            .find(|p| p.file_name() == Some(OsStr::new("build")))
            .and_then(Path::parent)
            .unwrap_or_else(|| panic!("OUT_DIR not inside a build dir: {}", out_dir.display()));
        let target_lib_dir = target_dir.join("lib");

        // Copy the library there so 'cargo run' works via the $ORIGIN/lib rpath.
        // Fail loudly: a silent failure here only surfaces later as a loader error.
        fs::create_dir_all(&target_lib_dir)
            .unwrap_or_else(|e| panic!("failed to create {}: {e}", target_lib_dir.display()));
        copy_dir_all(dir, &target_lib_dir).unwrap_or_else(|e| {
            panic!(
                "failed to copy fmod libraries to {}: {e}",
                target_lib_dir.display()
            )
        });

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
