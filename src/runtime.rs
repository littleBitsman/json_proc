use memchr::memchr;
use std::{collections::BTreeMap, error::Error as ErrorTrait, fmt::{Debug, Display, Formatter, Result as FmtResult}};

type Map<K, V> = BTreeMap<K, V>;

pub struct Location {
    line: usize,
    col: usize
}
impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "line {} column {}", self.line, self.col)
    }
}

pub struct Error {
    err: String,
    loc: Option<Location>
}
impl Error {
    pub(crate) fn new<S: ToString>(err: S, loc: Option<Location>) -> Self {
        Self {
            err: err.to_string(),
            loc
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.err)?;
        if let Some(ref loc) = self.loc {
            write!(f, " at {}", loc)?;
        }
        Ok(())
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}
impl ErrorTrait for Error {}

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Map<String, JsonValue>),
}

struct JsonParser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> JsonParser<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }

    pub fn parse(mut self) -> Result<JsonValue, Error> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.position != self.input.len() {
            return Err(Error::new("trailing characters", Some(self.location())));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, Error> {
        match self.peek() {
            Some(b'n') => self.parse_null(),
            Some(b't') | Some(b'f') => self.parse_bool(),
            Some(b'"') => self.parse_string(),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            Some(b'0'..=b'9') | Some(b'-') => self.parse_number(),
            Some(a) => Err(Error::new(format!("unexpected character {}", char::from_u32(a as u32).unwrap()), Some(self.location()))),
            None => Err(Error::new("unexpected EOF", None))
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, Error> {
        self.expect(b"null")?;
        Ok(JsonValue::Null)
    }

    fn parse_bool(&mut self) -> Result<JsonValue, Error> {
        if self.expect(b"true").is_ok() {
            Ok(JsonValue::Bool(true))
        } else if self.expect(b"false").is_ok() {
            Ok(JsonValue::Bool(false))
        } else {
            Err(Error::new("unexpected tokens", Some(self.location())))
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, Error> {
        self.advance(); // Skip initial quote
        let start = self.position;

        // Find the position of the next `"` character
        if let Some(pos) = memchr(b'\"', &self.input[start..]) {
            self.position = start + pos;
            let s = String::from_utf8(self.input[start..self.position].to_vec()).unwrap();
            self.advance(); // Skip closing quote
            Ok(JsonValue::String(s))
        } else {
            Err(Error::new("unterminated string", Some(self.location())))
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, Error> {
        let start = self.position;
        while let Some(b) = self.peek() {
            if !b.is_ascii_digit() && b != b'.' && b != b'-' && b != b'+' && b != b'e' && b != b'E'
            {
                break;
            }
            self.advance();
        }
        let num_str = std::str::from_utf8(&self.input[start..self.position]).unwrap();
        let number = num_str.parse().map_err(|_| Error::new("invalid number", Some(self.location())))?;
        Ok(JsonValue::Number(number))
    }

    fn parse_array(&mut self) -> Result<JsonValue, Error> {
        self.advance(); // Skip '['
        let mut elements = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b']') {
            self.advance(); // Empty array
            return Ok(JsonValue::Array(elements));
        }
        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.advance(),
                Some(b']') => {
                    self.advance();
                    break;
                }
                Some(ch) => return Err(Error::new(format!("unexpected character {} in array", char::from_u32(ch as u32).unwrap()), Some(self.location()))),
                None => return Err(Error::new("unterminated array", Some(self.location())))
            };
            self.skip_whitespace();
        }
        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, Error> {
        self.advance(); // Skip '{'
        let mut map = Map::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.advance(); // Empty object
            return Ok(JsonValue::Object(map));
        }
        loop {
            let key = if let JsonValue::String(s) = self.parse_string()? {
                s
            } else {
                return Err(Error::new("expected string literal for key in object", Some(self.location())));
            };
            self.skip_whitespace();
            if self.advance() != Some(b':') {
                return Err(Error::new("expected `:`", Some(self.location())))
            }
            self.skip_whitespace();
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => self.advance(),
                Some(b'}') => {
                    self.advance();
                    break;
                }
                Some(ch) => return Err(Error::new(format!("unexpected character {} in object", char::from_u32(ch as u32).unwrap()), Some(self.location()))),
                None => return Err(Error::new("unterminated object", Some(self.location())))
            };
            self.skip_whitespace();
        }
        Ok(JsonValue::Object(map))
    }

    fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !b.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn expect(&mut self, expected: &[u8]) -> Result<(), Error> {
        if self.input[self.position..].starts_with(expected) {
            self.position += expected.len();
            Ok(())
        } else {
            Err(Error::new("unexpected tokens", Some(self.location())))
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        if self.position < self.input.len() {
            let b = self.input[self.position];
            self.position += 1;
            Some(b)
        } else {
            None
        }
    }

    fn location(&self) -> Location {
        let mut line = 1;
        let mut col = 1;

        for &byte in &self.input[..self.position] {
            match byte {
                b'\n' => {
                    line += 1;
                    col = 1; // Reset column at the start of each new line
                }
                _ => col += 1,
            }
        }

        Location { line, col }
    }
}

pub fn parse<S: ToString>(s: S) -> Result<JsonValue, Error> {
    let string = s.to_string();
    JsonParser::new(&string).parse()
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value as SerdeJsonValue};
    use std::{
        hint::black_box,
        time::{Duration, Instant},
    };

    use super::*;

    fn bench_custom(s: &str) -> Duration {
        let start = Instant::now();
        let result = parse(s);
        let time = start.elapsed();
        match result {
            Ok(_) => println!("Custom took {:?}", start.elapsed()),
            Err(e) => eprintln!("Error: {e}"),
        }
        time
    }

    fn bench_serde_json(s: &str) -> Duration {
        let start = Instant::now();
        let result = from_str::<SerdeJsonValue>(s);
        let time = start.elapsed();
        match result {
            Ok(_) => println!("Serde took {:?}", start.elapsed()),
            Err(e) => eprintln!("Error: {e}"),
        }
        time
    }

    #[test]
    fn test() {
        const TRIALS: u32 = 10;
        const STRING: &str = r#"  {  "key": [null, true, 123, "text but long with \nescapes"], "nested": {"nestedKey": false}}"#;

        let average_custom = {
            let mut total = Duration::from_micros(0);
            for _ in 0..TRIALS {
                *black_box(&mut total) += black_box(bench_custom(black_box(STRING)))
            }
            total / TRIALS
        };
        
        let average_serde = {
            let mut total = Duration::from_micros(0);
            for _ in 0..TRIALS {
                *black_box(&mut total) += black_box(bench_serde_json(black_box(STRING)))
            }
            total / TRIALS
        };

        println!("Average custom: {:?}", average_custom);
        println!("Average serde: {:?}", average_serde);
    }
}
