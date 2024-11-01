fn ting(value: f32) -> f32 {
    value
}

fn main() {
    tests::test()
}

mod tests {
    use std::time::Instant;

    use super::*;
    use json_proc::*;

    #[cfg(test)]
    struct Fib(u128, u128);
    #[cfg(test)]
    impl Fib {
        const fn new() -> Self {
            Self(0, 0)
        }
    }
    #[cfg(test)]
    impl Default for Fib {
        #[inline(always)]
        fn default() -> Self {
            Self::new()
        }
    }
    #[cfg(test)]
    impl Iterator for Fib {
        type Item = u128;

        /// Gets the next number in the Fibonacci sequence.
        /// This will always return a [`Option::Some`].
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 0 && self.1 == 0 {
                self.0 = 1;
                Some(0)
            } else {
                let value = self.0 + self.1;
                self.0 = self.1;
                self.1 = value;
                Some(value)
            }
        }
    }

    #[test]
    fn fib() {
        const REPETITIONS: u32 = 120;
        let big_start = Instant::now();
        let mut fib = Fib::new();
        for _ in 0..(REPETITIONS - 1) {
            fib.next().unwrap();
        }
        let final_v = fib.next().unwrap();
        let time = big_start.elapsed();
        println!("per: {:?} | total: {:?} | end: {final_v}", time / REPETITIONS, time)
    }

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
            serde_json::to_string(&serde_json::json!($($json)+)).unwrap()
        };
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

        println!(
            "{}",
            json!({
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
                e2: false,
                es2: format!("hello: {} {hello}", "world!", hello = value),
                test22: strc,
                test2Enum: [
                    Test2::Hello { hello: String::from("this is Hello") },
                    Test2::Two(String::from("this is 2"), 2),
                ],
                array: Tuple(Test2::Hello { hello: String::from("this is some nested stuff") }, 0, String::from("directly in tuple"))
            })
        );

        // println!("{}", core::any::type_name_of_val(&None::<String>));
    }

    #[test]
    fn bench() {
        let hello = String::from("bad");
        let start = Instant::now();
        serde_json_str!({
            "hello": String::from("thisIsAString"),
            "struct": {
                "yes": String::new()
            },
            hello.clone(): hello
        });
        println!("{:?}", start.elapsed());
        let start = Instant::now();
        let _ = json!({
            "hello": String::from("thisIsAString"),
            "struct": Test {
                yes: String::new(),
                test: usize::MAX
            },
            bad: hello
        });
        println!("{:?}", start.elapsed());
    }

    #[test]
    fn test_basic_key_value_pairs() {
        let json_str = json!({
            "name": "John Doe",
            "age": 30,
            "active": true
        });
        assert_eq!(
            serde_json_str!(json_str),
            serde_json_str!(r#"{"name":"John Doe","age":30,"active":true}"#)
        );
    }

    #[test]
    fn test_string_from() {
        let json_str = json!({
            "greeting": String::from("Hello, world!"),
        });
        assert_eq!(
            serde_json_str!(json_str),
            serde_json_str!(r#"{"greeting":"Hello, world!"}"#)
        );
    }

    #[test]
    fn test_nested_json() {
        let json_str = json!({
            "user": {
                "name": "Jane Doe",
                "age": 25,
            },
            "is_admin": false,
        });
        assert_eq!(
            serde_json_str!(json_str),
            serde_json_str!(r#"{"user":{"name":"Jane Doe","age":25},"is_admin":false}"#)
        );
    }

    #[allow(dead_code)]
    fn test_json_with_escape_characters() {
        let json_str = json!({
            "message": "This is a \"quoted\" word.",
            "newline": "First line.\nSecond line.",
        });
        assert_eq!(serde_json_str!(json_str), serde_json_str!("{\"message\":\"This is a \\\"quoted\\\" word.\",\"newline\":\"First line.\\nSecond line.\"}"));
    }

    #[test]
    fn test_empty_json() {
        let json_str = json!({});
        assert_eq!(serde_json_str!(json_str), serde_json_str!(r#"{}"#));
    }

    #[test]
    fn test_complex_expressions() {
        let number = 42;
        let json_str = json!({
            "answer": number,
            "text": String::from("The answer is"),
        });
        assert_eq!(
            serde_json_str!(json_str),
            serde_json_str!(r#"{"answer":42,"text":"The answer is"}"#)
        );
    }
}
