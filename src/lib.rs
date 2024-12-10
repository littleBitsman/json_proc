#![doc = include_str!("../README.md")]

extern crate json_proc_macro;

mod json_trait;

pub use json_proc_macro::{json, ToJson};
pub use json_trait::ToJson;
