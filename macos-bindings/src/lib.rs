#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(unaligned_references)]
#![allow(clippy::all)]

#[cfg(target_os = "macos")]
include!(concat!(env!("OUT_DIR"), "/nsworkspace.rs"));
