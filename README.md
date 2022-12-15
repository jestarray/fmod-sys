# fmod-sys

Bindgen generated wrappers for FMOD

This library is aiming to be an idiomatic "-sys" version of wrappers for [FMOD](https://fmod.com).

The library itself is licensed using the [MIT license](./LICENSE), but the `fmodapi` folder contains headers from the FMOD API download, and those files are licensed under the [FMOD License](./fmodapi/LICENSE.TXT).

# Usage

You **MUST** register your product with FMOD and comply with their licensing terms before shipping your commercial product.

While I'm still getting this testing, you can use the `git` feature of Cargo.toml dependencies to use this crate:

```toml
[dependencies]
fmod-sys = { git = "https://github.com/jestarray/fmod-sys.git" }
```

You will need to put the fmod libraries in the root directory of your project.

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


# Building

To update the headers, just copy them into the fmodapi folder, make sure the LICENSE.TXT doesn't need to be updated, and then the build.rs script should automatically generate the new bindings.

## Mac OS Notes

When attempting to run an application linking against libfmod.dylib without signing it, you will be prompted for a security warning. Find the file in the Finder, right click on it and choose Open. It makes no sense to do this, but it will prompt you just like it would an unsigned application -- Click open and it will launch the Terminal app and do nothing.

However, once you've done this step, you can run the application until you replace the library wth a new version.

## Windows Notes:
The `_vc` suffix on the `.lib` files for windows need to be removed. Stick all the `.dll` and `.lib` in the root folder of your project. For shipping your game you only need to include `fmod.dll` and `fmodstudio.dll`

## Linux Notes:
For linux you may need to tell rust where to look for the libfmod files. 
1. Stuff the libfmod files in `/usr/lib` (easiest way)
2. Add the env variable in your `.bash_profile` `export LD_LIBRARY_PATH="PATH/TO/FMODLIBS"`

# todo
- put all the fmod.dlls into a lib folder. For some reason I can't get bindgen to look in the folder(seems like we cant really for windows?)
- examples