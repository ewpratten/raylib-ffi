use std::path::PathBuf;

use bindgen::Bindings;

fn generate_bindings(header_file: &str) {

    // Generate the data
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write `src/bindings.rs`
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn main() {
    // Files to watch that should trigger a rebuild
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Generate bindings
    generate_bindings("src/wrapper.h");
}
