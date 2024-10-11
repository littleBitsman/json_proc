#![expect(dead_code)]
#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result as SynResult},
    parse_macro_input,
    spanned::Spanned,
    token, Expr, Ident, LitBool, LitStr, Token,
};

enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(LitStr),
    Number(Expr),
    Bool(bool),
    Expr(Expr),
    #[allow(dead_code)]
    Null,
}

struct JsonKeyValue {
    key: String,
    value: JsonValue,
    key_span: Span,
}

struct JsonObject {
    pairs: Vec<JsonKeyValue>,
    span: Span,
}

struct JsonArray {
    elements: Vec<JsonValue>,
    span: Span,
}

impl Parse for JsonKeyValue {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse the key as a string
        let (key, span) = if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            (ident.to_string(), ident.span())
        } else {
            let litstr = input.parse::<LitStr>()?;
            (litstr.value(), litstr.span())
        };
        input.parse::<Token![:]>()?;

        // Parse the value (could be any JsonValue)
        let value: JsonValue = input.parse()?;
        Ok(JsonKeyValue {
            key,
            value,
            key_span: span,
        })
    }
}

impl Parse for JsonObject {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        let _ = braced!(content in input);
        let mut pairs = Vec::new();

        while !content.is_empty() {
            let pair: JsonKeyValue = content.parse()?;
            pairs.push(pair);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        {
            let lint_setting =
                lints::get_lint_level(lints::Lint::DuplicateKeys).unwrap_or_default();
            let mut map: Vec<(String, Span)> = Vec::new();
            for JsonKeyValue { key, key_span, .. } in pairs.iter() {
                if let (Some((_, span2)), Some(level)) = (
                    map.iter().find(|(key2, _)| key2.clone() == key.clone()),
                    lint_setting.level(),
                ) {
                    let mut d = Diagnostic::new(level, format!("duplicate key `{key}` in object"));
                    d.set_spans(key_span.unwrap());
                    d = d.span_note(span2.unwrap(), "key first defined here");
                    d.emit();
                } else {
                    map.push((key.clone(), key_span.clone()));
                }
            }
        }

        Ok(JsonObject {
            pairs,
            span: input.span(),
        })
    }
}

impl Parse for JsonArray {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        let _brackets = bracketed!(content in input);
        let mut elements = Vec::new();

        while !content.is_empty() {
            let elem: JsonValue = content.parse()?;
            elements.push(elem);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(JsonArray {
            elements,
            span: input.span(),
        })
    }
}

impl JsonValue {
    fn span(&self) -> Option<Span> {
        match self {
            JsonValue::Object(json_object) => Some(json_object.span.clone()),
            JsonValue::Array(json_array) => Some(json_array.span),
            JsonValue::String(lit_str) => Some(lit_str.span()),
            JsonValue::Number(expr) => Some(expr.span()),
            JsonValue::Bool(_) => None,
            JsonValue::Expr(expr) => Some(expr.span()),
            JsonValue::Null => None,
        }
    }
}

impl Parse for JsonValue {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(LitStr) {
            let s: LitStr = input.parse()?;
            Ok(JsonValue::String(s))
        } else if input.peek(LitBool) {
            Ok(JsonValue::Bool(input.parse::<LitBool>()?.value))
        } else if input.peek(Ident) || input.peek(syn::token::Paren) {
            let expr: Expr = input.parse()?;
            Ok(JsonValue::Expr(expr))
        } else if input.peek(token::Brace) {
            let obj: JsonObject = input.parse()?;
            Ok(JsonValue::Object(obj))
        } else if input.peek(token::Bracket) {
            let arr: JsonArray = input.parse()?;
            Ok(JsonValue::Array(arr))
        } else {
            let expr: Expr = input.parse()?;
            Ok(JsonValue::Number(expr))
        }
    }
}

impl quote::ToTokens for JsonValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            JsonValue::Object(obj) => obj.to_tokens(tokens),
            JsonValue::Array(arr) => arr.to_tokens(tokens),
            JsonValue::String(litstr) => quote! { format!("\"{}\"", #litstr) }.to_tokens(tokens),
            JsonValue::Number(expr) => expr.to_tokens(tokens),
            JsonValue::Bool(b) => {
                let b = *b;
                let token = quote! { #b };
                token.to_tokens(tokens);
            }
            JsonValue::Null => quote! { "null" }.to_tokens(tokens),
            JsonValue::Expr(expr) => {
                quote! {
                    {
                        let value = #expr;

                        json_proc::ToJson::to_json_string(&value)
                    }
                }.to_tokens(tokens);
            }
        }
    }
}

// Implementing quote for JsonObject to generate valid Rust code
impl quote::ToTokens for JsonObject {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pairs = &self.pairs;
        let mut pairs_tokens = Vec::new();
        for pair in pairs {
            let key = &pair.key;
            let value = &pair.value;
            pairs_tokens.push(quote! {
                format!("\"{}\": {}", #key, #value)
            });
        }
        let output = quote! {
            format!("{{ {} }}", {
                let vec: Vec<String> = vec![#(#pairs_tokens.to_string()),*];
                vec
            }.join(","))
        };
        output.to_tokens(tokens);
    }
}

impl quote::ToTokens for JsonArray {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let elements = &self.elements;
        let mut elements_tokens = Vec::new();
        for elem in elements {
            elements_tokens.push(quote! {
                #elem
            });
        }
        let output = quote! {
            format!("[{}]", vec![#(#elements_tokens.to_string()),*].join(","))
        };
        output.to_tokens(tokens);
    }
}

/// Lints and properly formats a JSON object, array, or value.
///
/// This proc-macro supports:
/// - all literals (integers, floats, [`&str`])
/// - [`String`]
/// - any expression that evaluates to a [`impl Display`]
///
/// If you are looking for custom serialization traits, macros,
/// and functions, use `serde_json` and `serde` instead.
///
/// Examples:
///
/// Serializing an object:
/// ```no_run
/// fn obj<D: Display>(input: D) -> String {
///     json!({
///         "hello": "world!",
///         thisDidntNeedQuotes: "wow!",
///         anExpression: input // this will essentially become `format!("{input}")`
///     })
/// }
/// ```
///
/// Serializing an array:
/// ```no_run
/// fn arr<D: Display>(input: D) -> String {
///     json!([
///         input,
///         (2 + 11) as f32 / 2.0,
///         "literal"
///     ])
/// }
/// ```
///
/// [`&str`]: str
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let json_value: JsonValue = parse_macro_input!(input as JsonValue);

    let output = quote! {
        #json_value
    };

    output.into()
}

mod lints {
    use std::{collections::BTreeMap, sync::Mutex};

    use proc_macro::TokenStream;
    use proc_macro2::TokenStream as TokenStream2;
    use syn::{
        parse::{Parse, ParseStream},
        parse_macro_input, Error as SynError, Ident, Result as SynResult, Token,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Lint {
        DuplicateKeys,
    }

    impl TryFrom<&str> for Lint {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value.to_string().as_str() {
                "duplicate_keys" => Ok(Self::DuplicateKeys),
                _ => Err(()),
            }
        }
    }

    impl Parse for Lint {
        fn parse(input: ParseStream) -> SynResult<Self> {
            let ident = input.parse::<Ident>()?;
            Self::try_from(ident.to_string().as_str())
                .map_err(|_| SynError::new(input.span(), "invalid lint"))
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Level {
        Allow,
        Warning,
        #[default]
        Error,
    }
    impl Level {
        pub fn level(self) -> Option<proc_macro::Level> {
            match self {
                Self::Allow => None,
                Self::Warning => Some(proc_macro::Level::Warning),
                Self::Error => Some(proc_macro::Level::Error),
            }
        }
    }

    static LINTS: Mutex<BTreeMap<Lint, Level>> = Mutex::new(BTreeMap::new());

    pub fn get_lint_level(lint: Lint) -> Option<Level> {
        LINTS.lock().unwrap().get(&lint).copied()
    }

    struct LintList(Vec<Lint>);
    impl Parse for LintList {
        fn parse(input: ParseStream) -> SynResult<Self> {
            let mut lints = Vec::new();

            while !input.is_empty() {
                lints.push(input.parse::<Lint>()?);
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
            }

            Ok(Self(lints))
        }
    }

    pub fn allow(input: TokenStream, item: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let mut lock = LINTS.lock().unwrap();
        for lint in lints {
            lock.insert(lint, Level::Allow);
        }

        item
    }

    pub fn warn(input: TokenStream, item: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let item = TokenStream2::from(item);

        let mut lock = LINTS.lock().unwrap();
        for lint in lints {
            lock.insert(lint, Level::Warning);
        }

        item.into()
    }

    pub fn error(input: TokenStream, item: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let item = TokenStream2::from(item);

        let mut lock = LINTS.lock().unwrap();
        for lint in lints {
            lock.insert(lint, Level::Error);
        }

        item.into()
    }
}

/// Sets the lint level of all listed lints to none.
///
/// Note that this will emit a warning if a lint level
/// was previously defined.
///
/// Example:
/// ```no_run
/// #[json::allow_json(duplicate_keys)]
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will not do ANYTHING
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn allow_json(input: TokenStream, item: TokenStream) -> TokenStream {
    lints::allow(input, item)
}

/// Sets the lint level of all listed lints to warning.
///
/// Note that this will emit a warning if a lint level
/// was previously defined.
///
/// Example:
/// ```no_run
/// #[json::warn_json(duplicate_keys)]
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will warn
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn warn_json(input: TokenStream, item: TokenStream) -> TokenStream {
    lints::warn(input, item)
}

/// Sets the lint level of all listed lints to error.
///
/// Note that this will emit a warning if a lint level
/// was previously defined.
///
/// Example:
/// ```no_run
/// #[json::error_json(duplicate_keys)]
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will error
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn error_json(input: TokenStream, item: TokenStream) -> TokenStream {
    lints::error(input, item)
}

/// Sets the lint level of all listed lints to error.
/// This attribute is an alias for [`error_json`].
///
/// Note that this will emit a warning if a lint level
/// was previously defined.
///
/// Example:
/// ```no_run
/// #[json::deny_json(duplicate_keys)]
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will error
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn deny_json(input: TokenStream, item: TokenStream) -> TokenStream {
    error_json(input, item)
}
