pub trait ToJson {
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
    f32 f64,
}

macro_rules! string_json_impl {
    { $($ty:ty $(,)?)* } => {
        $(
            impl ToJson for $ty {
                fn to_json_string(&self) -> String {
                    format!(r#""{}""#, self)
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
            None => String::from("null")
        }
    }
}

impl ToJson for () {
    fn to_json_string(&self) -> String {
        String::from("null")
    }
}