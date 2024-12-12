#![doc = include_str!("../README.md")]
#![cfg_attr(compiler = "nightly", feature(never_type, ascii_char))]

extern crate json_proc_macro;

mod json_trait;

pub use json_proc_macro::{json, ToJson};
pub use json_trait::ToJson;
