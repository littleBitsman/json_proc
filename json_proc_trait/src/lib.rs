//! This crate only provides the [`ToJson`] trait.
//!
//! See the documentation of `json_proc` for more info.
//!
//! [`ToJson`]: crate::ToJson

/// Trait that converts a type to a JSON string.
///
/// `json_proc_macro` (exported by `json_proc`) provides
/// a macro that derives this trait.
pub trait ToJson {
    /// Converts self to a JSON string.
    ///
    /// Implementations of this should not fail.
    fn to_json_string(&self) -> String;
}

macro_rules! display_json_impl {
    { $($ty:ty $(,)?)* } => {
        $(
            impl ToJson for $ty {
                fn to_json_string(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    };
}

display_json_impl! {
    u8 u16 u32 u64 usize,
    i8 i16 i32 i64 isize,
    f32 f64
}

// FIXME: this doesn't correctly handle newlines
// and other escaped characters
// (AFAIK its only '\n')
macro_rules! string_json_impl {
    { $($ty:ty $(,)?)* } => {
        $(
            impl ToJson for $ty {
                fn to_json_string(&self) -> String {
                    format!(r#""{}""#, self.replace('"', "\\\""))
                }
            }
        )*
    };
}

string_json_impl! {
    String
    &str
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json_string(&self) -> String {
        match self {
            Some(t) => t.to_json_string(),
            None => String::from("null"),
        }
    }
}

impl ToJson for () {
    fn to_json_string(&self) -> String {
        String::from("null")
    }
}
