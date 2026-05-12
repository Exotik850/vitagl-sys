use std::env;
use std::path::PathBuf;

fn main() {
    let Ok(sdk) = env::var("VITASDK").map(PathBuf::from) else {
        println!("cargo:warning=$VITASDK not set!");
        return;
    };
    let lib_dir = sdk.join("arm-vita-eabi").join("lib");

    #[cfg(feature = "bindgen")]
    {
        let vitagl_header = sdk.join("arm-vita-eabi/include/vitaGL.h");
        let bindings = bindgen::Builder::default()
            .header(vitagl_header.to_string_lossy())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .clang_arg("--target=arm-vita-eabi")
            .clang_arg(format!("--sysroot={}", sdk.to_string_lossy()))
            .clang_arg("-I")
            .clang_arg(sdk.join("arm-vita-eabi/include").to_string_lossy())
            .generate()
            .expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    // Do not link libraries for docs.rs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=vitaGL");
    println!("cargo:rustc-link-lib=static=vitashark");
    println!("cargo:rustc-link-lib=static=mathneon");
    println!("cargo:rustc-link-lib=static=SceShaccCg_stub");
    println!("cargo:rustc-link-lib=static=SceGxm_stub");
    println!("cargo:rustc-link-lib=static=SceKernelDmacMgr_stub");
    println!("cargo:rustc-link-lib=static=SceDisplay_stub");
    println!("cargo:rustc-link-lib=static=SceCommonDialog_stub");
    println!("cargo:rustc-link-lib=static=SceAppMgr_stub");
    println!("cargo:rustc-link-lib=static=SceShaccCgExt");
    println!("cargo:rustc-link-lib=static=taihen_stub");
    // vitaGL is built as C++, so we must link the C++ runtime.
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=supc++");
}
