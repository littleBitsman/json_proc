fn ting(value: f32) -> f32 {
    value
}

fn main() {
    tests::test()
}

mod tests {
    use std::{hint::black_box, time::Instant};

    use super::*;
    use json_proc::*;

    #[derive(ToJson)]
    struct Test<T: json_proc::ToJson> {
        yes: String,
        test: T,
    }

    #[derive(ToJson)]
    struct Tuple<T>(T, u32, String);

    #[derive(ToJson)]
    enum Test2 {
        Hello { hello: String },
        Two(String, u8),
    }

    #[cfg(test)]
    macro_rules! serde_json_str {
        ($($json:tt)+) => {
            ::serde_json::to_string(&::serde_json::json!($($json)+)).unwrap()
        };
    }

    #[cfg(test)]
    fn json_like_serde<S: ToString>(s: S) -> String {
        serde_json::from_str::<serde_json::Value>(&s.to_string()).unwrap().to_string()
    }

    #[cfg(test)]
    macro_rules! check_tt {
        ($($tt:tt)+) => {{
            let lhs = json_like_serde(::json_proc::json!($($tt)+));
            let rhs = serde_json_str!($($tt)+);
            if cfg!(debug_assertions) {
                println!("LHS: {lhs}");
                println!("RHS: {rhs}");
            }
            assert_eq!(lhs, rhs);
        }};
    }

    #[test]
    pub fn thingy() {
        check_tt!({
            "hlelo": "hi"
        })
    }

    #[cfg_attr(test, test)]
    pub fn test() {
        println!("{}", json!({}));
        let value = String::from("ga");
        let other_value = 1f32 / 32f32;

        let strc = Test {
            yes: String::from("hello"),
            test: u32::MAX,
        };
        
        let start = Instant::now();
        let finished = json!({
            "hello": (2 + 4) as f32 + other_value + (b'e' as f32) + ting(100.0),
            "e": String::from("hello"),
            "test": None::<()>,
            "not": "trueStr",
            "embedded_array": [
                1,2,
                5
                ,10,
                {
                    "heck": true
                }

            ]
            ,
            "e2": false,
            "fake": format!("Can I see the syntax highlights pls {}", false),
            "es2": format!("hello: {} {hello}", "world!", hello = value),
            "test22": strc,
            "test2Enum": [
                Test2::Hello { hello: String::from("this is Hello") },
                Test2::Two(String::from("this is 2"), 2),
            ],
            "array": Tuple(Test2::Hello { hello: String::from("this is some nested stuff") }, 0, String::from("directly in tuple")),
            "a_null": null,
            value: value
        });
        let dur = start.elapsed();
        println!("{finished}\nTook: {dur:?}");

        // println!("{}", core::any::type_name_of_val(&None::<String>));
    }

    #[test]
    fn bench() {
        let hello = String::from("bad");
        let start = Instant::now();
        black_box(serde_json_str!({
            "hello": String::from("thisIsAString"),
            "struct": {
                "yes": String::new()
            },
            hello.clone(): hello,
            "null": null
        }));
        println!("{:?}", start.elapsed());
        let start = Instant::now();
        black_box(json!({
            "hello": String::from("thisIsAString"),
            "struct": Test {
                yes: String::new(),
                test: usize::MAX
            },
            "bad": hello,
            "null": null,
            hello: "hi"
        }));
        println!("{:?}", start.elapsed());
    }

    #[test]
    fn test_basic_key_value_pairs() {
        check_tt!({
            "name": "John Doe",
            "age": 30,
            "active": true
        })
    }

    #[test]
    fn test_string_from() {
        check_tt!({
            "greeting": String::from("Hello, world!"),
        })
    }

    #[test]
    fn test_nested_json() {
        check_tt!({
            "user": {
                "name": "Jane Doe",
                "age": 25,
            },
            "is_admin": false,
        })
    }

    #[allow(dead_code)]
    fn test_json_with_escape_characters() {
        check_tt!({
            "message": "This is a \"quoted\" word.",
            "newline": "First line.\nSecond line.",
        });
    }

    #[test]
    fn test_empty_json() {
        check_tt!({});
    }

    #[test]
    fn test_complex_expressions() {
        let number = 42;
        check_tt!({
            "answer": number,
            "text": String::from("The answer is"),
        })
    }
}
