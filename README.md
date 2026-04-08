# fmod-sys

Bindgen generated wrappers for FMOD

This library is aiming to be an idiomatic "-sys" version of wrappers for [FMOD](https://fmod.com).

The library itself is licensed using the [MIT license](./LICENSE), but the `fmodapi` folder contains headers from the FMOD API download, and those files are licensed under the [FMOD License](./fmodapi/LICENSE.TXT).

# Usage

You **MUST** register your product with FMOD and comply with their licensing terms before shipping your commercial product.

## Linux
On Linux, the build script is configured to look for the fmod libs in a `lib` folder in your game's root directory, e.g `my_game/lib/<libfmod.so>`. You'll need to tell cargo where to look for the libraries and there are 2 ways of doing so:

### Recommended(my_game/./cargo/config.toml)
Create the following in your game's project root: `my_game/.cargo/config.toml`
```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-Wl,-rpath,$ORIGIN/lib"]
```

### Adding to your games build script
or if you use a build script, e.g: `my_game/build.rs`:
```rs
println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
```

Note: The build script will also copy `fmod_sys/lib_x64_linux` files to `target/{debug,release}/lib` folder so you can run your game without hassle.

### Distrbuting on Linux
When distributing your game's binary, you will need to put the fmod libraries in a `lib` folder in the **root directory** of your project, e.g `my_game/lib/<libfmod.so>`. Your game's executable will look relative to where it is for a `lib` folder. You can grab them from `fmod_sys/lib_x64_linux/`

## Windows
Stick all fmod `.dll` and `.lib` in the root folder of your project. These can be found in `fmod_sys/lib_windows_x64` or `C:\Program Files (x86)\FMOD SoundSystem\FMOD Studio API Windows\api\lib`(presuming windows install).

For shipping your game on windows, you only need to include `fmod.dll` and `fmodstudio.dll`(if you use it) to be placed where your game's `.exe` is.

# Enabling Studio APIs

By default, the core library and its API is all that is exported. If you want to enable the FMOD Studio API, enable the `studio` feature:

```toml
[dependencies]
fmod-sys = { git = "https://github.com/jestarray/fmod-sys.git", features = ["studio"] }
```

# Enabling Debug logging features of fmod

```toml
[dependencies]
fmod-sys = { git = "https://github.com/jestarray/fmod-sys.git", features = ["Debug"] }
```

## Mac OS Notes

When attempting to run an application linking against libfmod.dylib without signing it, you will be prompted for a security warning. Find the file in the Finder, right click on it and choose Open. It makes no sense to do this, but it will prompt you just like it would an unsigned application -- Click open and it will launch the Terminal app and do nothing.

However, once you've done this step, you can run the application until you replace the library wth a new version.


## Developer Notes:
### Upgrading fmod
To update the headers, just copy them into the fmodapi folder, and then the build.rs script should automatically generate the new bindings in `target/{debug,release}/build/fmod-sys-{hash}/out`. Copy and paste these bindings to `./pre-gen`

## Linux Notes:
If you run into issues with linux still not being able to find libfmod, here are some work arounds:
1. Stuff the libfmod files in `/usr/lib` (easiest way)
2. Add the env variable in your `.bash_profile` `export LD_LIBRARY_PATH="PATH/TO/FMODLIBS"`

# todo
- put all the fmod.dlls into a lib folder. For some reason I can't get bindgen to look in the folder(seems like we cant really for windows?)
- examples