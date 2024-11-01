//! A proc-macro that evaluates JSON-like syntax to a
//! JSON string at compile time.
//!
//! (Better docs coming soon)

pub extern crate json_proc_macro;

mod json_trait;

pub use json_proc_macro::*;
pub use json_trait::ToJson;