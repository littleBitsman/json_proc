//! A proc-macro that evaluates JSON-like syntax to a
//! JSON string at compile time.
//!
//! If you are looking for custom serialization traits, macros,
//! and functions, use `serde_json` and `serde` instead.

pub extern crate json_proc_macro as macros;

mod json_trait;

pub use json_trait::ToJson;
pub use macros::*;
