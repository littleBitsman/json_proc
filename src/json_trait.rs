//! This module only provides the [`ToJson`] trait.
//!
//! See the documentation of [`json_proc`] for more info.
//!
//! [`ToJson`]: crate::ToJson
//! [`json_proc`]: https://docs.rs/json_proc/latest/json_proc

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

/// Trait that converts a type to a JSON string.
///
/// This trait has a [derive macro].
///
/// [derive macro]: https://docs.rs/json_proc/latest/json_proc/derive.ToJson.html
pub trait ToJson {
    /// Converts self to a JSON string.
    ///
    /// Implementations of this should not fail.
    #[must_use = "converting to a JSON string is often expensive and is not expected to have side effects"]
    fn to_json_string(&self) -> String;
}

macro_rules! display_json_impl {
    { $($ty:ty $(,)?)* } => {
        $(
            impl ToJson for $ty {
                #[inline]
                fn to_json_string(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

display_json_impl! {
    u8 u16 u32 u64 u128 usize,
    i8 i16 i32 i64 i128 isize,
    f32 f64,
    bool,
}

// FIXME: this doesn't correctly handle newlines
// and other escaped characters
// (AFAIK its only '\n')
impl ToJson for String {
    fn to_json_string(&self) -> String {
        let mut json = String::with_capacity(self.len() + 2);
        json.push('"');
        json.push_str(self);
        json.push('"');

        json
    }
}

impl ToJson for str {
    fn to_json_string(&self) -> String {
        let mut json = String::with_capacity(self.len() + 2);
        json.push('"');
        json.push_str(self);
        json.push('"');

        json
    }
}

impl ToJson for char {
    fn to_json_string(&self) -> String {
        let mut json = String::with_capacity(3);
        json.push('"');
        json.push(*self);
        json.push('"');

        json
    }
}

impl<T: ToJson> ToJson for Option<T> {
    #[inline]
    fn to_json_string(&self) -> String {
        match self {
            Some(t) => t.to_json_string(),
            None => String::from("null"),
        }
    }
}

impl<T, E> ToJson for Result<T, E>
where
    T: ToJson,
    E: ToJson,
{
    #[inline]
    fn to_json_string(&self) -> String {
        match self {
            Ok(t) => t.to_json_string(),
            Err(e) => e.to_json_string(),
        }
    }
}

impl ToJson for () {
    #[inline]
    fn to_json_string(&self) -> String {
        String::from("null")
    }
}

impl<T: ToJson> ToJson for &[T] {
    fn to_json_string(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(ToJson::to_json_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
impl<T: ToJson, const N: usize> ToJson for [T; N] {
    #[inline]
    fn to_json_string(&self) -> String {
        self.as_slice().to_json_string()
    }
}
impl<T: ToJson> ToJson for Vec<T> {
    #[inline]
    fn to_json_string(&self) -> String {
        self.as_slice().to_json_string()
    }
}

impl<K, V> ToJson for BTreeMap<K, V>
where
    K: ToString,
    V: ToJson,
{
    fn to_json_string(&self) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|(key, value)| format!(r#""{}":{}"#, key.to_string(), value.to_json_string()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<K, V> ToJson for HashMap<K, V>
where
    K: ToString,
    V: ToJson,
{
    fn to_json_string(&self) -> String {
        format!(
            "{{{}}}",
            self.iter()
                .map(|(key, value)| format!(r#""{}":{}"#, key.to_string(), value.to_json_string()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<T: ToJson> ToJson for BTreeSet<T> {
    fn to_json_string(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(ToJson::to_json_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<T: ToJson> ToJson for HashSet<T> {
    fn to_json_string(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(ToJson::to_json_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<T: ToJson> ToJson for VecDeque<T> {
    fn to_json_string(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(ToJson::to_json_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
