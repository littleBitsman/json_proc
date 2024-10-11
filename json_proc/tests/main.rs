mod main2;

#[cfg(test)]
mod tests {
    use json_proc::*;

    use super::main2::*;

    #[test]
    fn test_basic_key_value_pairs() {
        let json_str = json!({
            "name": "John Doe",
            "age": 30,
            "active": true
        });
        assert_eq!(json_str.to_string(), r#"{"name": "John Doe", "age": "30", "active": "true"}"#);
    }

    #[test]
    fn test_string_from() {
        let json_str = json!({
            "greeting": String::from("Hello, world!"),
        });
        assert_eq!(json_str.to_string(), r#"{"greeting": "Hello, world!"}"#);
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
        assert_eq!(json_str.to_string(), r#"{"user": {"name": "Jane Doe", "age": "25"}, "is_admin": "false"}"#);
    }

    #[test]
    fn test_json_with_escape_characters() {
        let json_str = json!({
            "message": "This is a \"quoted\" word.",
            "newline": "First line.\nSecond line.",
        });
        assert_eq!(json_str.to_string(), r#"{"message": "This is a \"quoted\" word.", "newline": "First line.\nSecond line."}"#);
    }

    #[test]
    fn test_empty_json() {
        let json_str = json!({});
        assert_eq!(json_str.to_string(), r#"{}"#);
    }

    #[test]
    fn test_complex_expressions() {
        let number = 42;
        let json_str = json!({
            "answer": number,
            "text": String::from("The answer is"),
        });
        assert_eq!(json_str.to_string(), r#"{"answer": "42", "text": "The answer is"}"#);
    }
}
