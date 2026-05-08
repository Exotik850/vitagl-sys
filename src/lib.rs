#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(nonstandard_style)]
#![doc = include_str!("../README.md")]

#[cfg(target_os = "vita")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


