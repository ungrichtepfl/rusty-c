extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=clibs/reverse.cpp");
    println!("cargo:rerun-if-changed=clibs/quicksort.cpp");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // C bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // C++ bindings
    let bindings_cpp = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings");

    bindings_cpp.write_to_file(out_path.join("bindings_cpp.rs")).expect("Couldn't write bindings!");
    

    // Build all c/c++ libs
    cc::Build::new().file("clibs/quicksort.c").compile("quicksort");
    cc::Build::new().cpp(true).file("clibs/reverse.cpp").compile("reverse");
}
