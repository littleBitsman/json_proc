//! A proc-macro that evaluates JSON-like syntax to a
//! JSON string at compile time.
//!
//! If you are looking for custom serialization traits, macros,
//! and functions, use `serde_json` and `serde` instead.

extern crate json_proc_macro;

mod json_trait;

pub use json_proc_macro::{json, ToJson};
pub use json_trait::ToJson;
