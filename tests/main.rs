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

    #[derive(ToJson)]
    struct Test {
        yes: String
    }

    #[derive(ToJson)]
    enum Test2 {
        Hello {
            hello: String
        },
        Two(String, u8)
    }

    #[cfg(test)]
    macro_rules! serde_json_str {
        ($($json:tt)+) => {
            serde_json::to_string(&serde_json::json!($($json)+)).unwrap()
        };
    }

    #[cfg_attr(test, test)]
    pub fn test() {
        let value = String::from("ga");
        let other_value = 1f32/32f32;

        let strc = Test { yes: String::from("hello") };
        
        println!("{}", json!({
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
            ]
        }));
    
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
                yes: String::new()
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
        assert_eq!(serde_json_str!(json_str), serde_json_str!(r#"{"name":"John Doe","age":30,"active":true}"#));
    }

    #[test]
    fn test_string_from() {
        let json_str = json!({
            "greeting": String::from("Hello, world!"),
        });
        assert_eq!(serde_json_str!(json_str), serde_json_str!(r#"{"greeting":"Hello, world!"}"#));
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
        assert_eq!(serde_json_str!(json_str), serde_json_str!(r#"{"user":{"name":"Jane Doe","age":25},"is_admin":false}"#));
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
        assert_eq!(serde_json_str!(json_str), serde_json_str!(r#"{"answer":42,"text":"The answer is"}"#));
    }
}
