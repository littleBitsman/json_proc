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
                #[inline]
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
    f32 f64,
    bool,

}

// FIXME: this doesn't correctly handle newlines
// and other escaped characters
// (AFAIK its only '\n')
macro_rules! string_json_impl {
    { $($ty:ty $(,)?)* } => {
        $(
            impl ToJson for $ty {
                #[inline]
                fn to_json_string(&self) -> String {
                    format!(r#""{}""#, self.to_string().replace('"', "\\\""))
                }
            }
        )*
    };
}

string_json_impl! {
    String
    &str char
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
    #[inline]
    fn to_json_string(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|v| v.to_json_string())
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