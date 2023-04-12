#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "enums")]
pub mod enums;

#[cfg(feature = "macros")]
#[macro_use]
pub mod macros;

#[cfg(feature = "colors")]
pub mod colors;