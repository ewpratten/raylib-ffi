#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// This module contains auto-generated Rust representations of raylib's enums.
pub mod enums {
    include!(concat!(env!("OUT_DIR"), "/enums.rs"));
}

/// This module contains auto-generated Rust representations of raylib's colors.
pub mod colors {
    include!(concat!(env!("OUT_DIR"), "/colors.rs"));
}

#[cfg(feature = "macros")]
#[macro_use]
pub mod macros;
