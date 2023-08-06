use std::{env, io::Error, path::Path, process::Command};

fn get_sdk_path() -> Result<String, Error> {
    let output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()?
        .stdout;
    Ok(String::from_utf8(output).unwrap().trim().to_string())
}

pub fn main() {
    let target = std::env::var("TARGET").unwrap();
    let sdk_path = get_sdk_path().unwrap();

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AppKit");

    let builder = bindgen::Builder::default()
        .header_contents(
            "NSWorkspace.h",
            "
            #include<AppKit/NSWorkspace.h>
            #include<AppKit/NSRunningApplication.h>
        ",
        )
        .clang_arg(format!("--target={}", target))
        .clang_args(&["-isysroot", sdk_path.as_ref()])
        .block_extern_crate(true)
        .objc_extern_crate(true)
        .clang_arg("-ObjC")
        .blocklist_item("objc_object");

    let bindings = builder.generate().unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    bindings
        .write_to_file(Path::new(&out_dir).join("nsworkspace.rs"))
        .unwrap();
}
