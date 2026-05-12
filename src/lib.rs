#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(nonstandard_style)]
#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(target_os = "vita"),
    doc = "\n\n# This crate provides bindings to the PlayStation Vita's native APIs. It is only intended to be used on the Vita platform, and will not work on other platforms."
)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
mod bindgen;
#[cfg(not(feature = "bindgen"))]
pub use bindgen::*;
