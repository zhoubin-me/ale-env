use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let root_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let ale_dir = root_dir.join("ale");
    let mut config = cmake::Config::new(&ale_dir);
    let des = config.build();

    println!("cargo:rustc-link-search=native={}/lib", des.display());
    println!("cargo:rustc-link-lib=ale_c");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!("-I{}/include", des.display()))
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++17")
        .enable_cxx_namespaces()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=z");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
