use std::{path::PathBuf, collections::HashSet};

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

/// Compiles raylib
fn compile_raylib(raylib_path: &str) {
    // Construct a config for running cmake
    let mut cmake_config = cmake::Config::new(raylib_path);
    let mut cmake_config = cmake_config
        .define("BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release");

    // Set the correct build profile
    #[cfg(debug_assertions)]
    {
        cmake_config = cmake_config.profile("Debug");
    }

    #[cfg(not(debug_assertions))]
    {
        cmake_config = cmake_config.profile("Release");
    }

    // Build the cmake project
    let destination = cmake_config.build();

    // Tell cargo where the libraries might be
    println!(
        "cargo:rustc-link-search=native={}",
        destination.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        destination.join("lib32").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        destination.join("lib64").display()
    );
}

/// Links libraries
fn link_libs() {
    // Handle windows libs
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    }
    // Handle MacOS
    else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
    // Handle UNIX
    else if cfg!(unix) {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=X11");
    }
    // Fail on other platforms
    else {
        panic!("Unsupported platform");
    }

    // Link raylib itself
    println!("cargo:rustc-link-lib=static=raylib");
}

/// Generates `bindings.rs` file
fn generate_bindings(header_file: &str) {
    // Tell bindgen to ignore these macros
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    // Generate the data
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros))
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

    // Compile raylib
    compile_raylib("third_party/raylib");

    // Link libraries
    link_libs();

    // Generate bindings
    generate_bindings("src/wrapper.h");
}
