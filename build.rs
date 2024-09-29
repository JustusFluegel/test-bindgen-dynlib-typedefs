use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=header.h");

    let bindings = bindgen::Builder::default()
        .header("header.h")
        .dynamic_library_name("TestLib")
        // set this if all items are always in the dynlib
        .dynamic_link_require_all(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindigns");
}
