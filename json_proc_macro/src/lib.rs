#![expect(dead_code)]
#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    braced, bracketed, parse::{Parse, ParseStream, Result as SynResult}, parse_macro_input, spanned::Spanned, token, Expr, Ident, ItemStruct, LitBool, LitStr, Member, Token
};

enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(LitStr),
    Number(Expr),
    Bool(bool),
    Expr(Expr),
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
            JsonValue::String(litstr) => quote!(format!("\"{}\"", #litstr)).to_tokens(tokens),
            JsonValue::Number(expr) => expr.to_tokens(tokens),
            JsonValue::Bool(b) => {
                let b = *b;
                let token = quote!(#b);
                token.to_tokens(tokens);
            }
            JsonValue::Null => quote!("null").to_tokens(tokens),
            JsonValue::Expr(expr) => {
                quote!(json_proc::ToJson::to_json_string(&(#expr))).to_tokens(tokens);
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
            pairs_tokens.push(quote!(format!("\"{}\":{}", #key, #value)));
        }
        let output = quote! {
            format!("{{{}}}", {
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
            elements_tokens.push(quote!(#elem));
        }
        let output = quote!(format!("[{}]", vec![#(#elements_tokens.to_string()),*].join(",")));
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
/// // You have to have the `ToJson` trait restriction since
/// // the json! macro uses ToJson. Should a struct not 
/// // implement ToJson, you can use the derive macro.
/// fn obj<J: json_proc::ToJson>(input: J) -> String {
///     json!({
///         "hello": "world!",
///         thisDidntNeedQuotes: "wow!",
///         // this will essentially become `format!("{}", input.to_json_string())`
///         anExpression: input 
///     })
/// }
/// ```
///
/// Serializing an array:
/// ```no_run
/// fn arr<J: json_proc::ToJson>(input: J) -> String {
///     json!([
///         input,
///         (2 + 11) as f32 / 2.0,
///         "literal"
///     ])
/// }
/// ```
///
/// [`&str`]: str
/// [`String`]: std::string::String
/// [`impl Display`]: std::fmt::Display
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let json_value = parse_macro_input!(input as JsonValue);

    quote!(#json_value).into()
}

mod lints {
    use std::{collections::BTreeMap, ops::Deref, sync::Mutex};

    use proc_macro::{Span, TokenStream};
    use syn::{
        parse::{Parse, ParseStream},
        parse_macro_input, Error as SynError, Ident, Result as SynResult, Token,
    };

    pub struct SpanWrapper(Span);
    impl Deref for SpanWrapper {
        type Target = Span;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

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
                .map_err(|_| SynError::new(ident.span(), "invalid lint"))
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

    pub fn get_lint_level<'a>(lint: Lint) -> Option<Level> {
        LINTS.lock().unwrap().get_mut(&lint).copied()
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

    pub fn allow(input: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let mut lock = LINTS.lock().unwrap();
        if lints.is_empty() {
            Span::call_site()
                .warning("no lint levels are being defined in this macro call")
                .help("remove this macro call")
                .emit();
        } else {
            for lint in lints {
                lock.insert(lint, Level::Allow);
            }
        }

        TokenStream::new()
    }

    pub fn warn(input: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let mut lock = LINTS.lock().unwrap();
        if lints.is_empty() {
            Span::call_site()
                .warning("no lint levels are being defined in this macro call")
                .help("remove this macro call")
                .emit();
        } else {
            for lint in lints {
                lock.insert(lint, Level::Warning);
            }
        }

        TokenStream::new()
    }

    pub fn error(input: TokenStream) -> TokenStream {
        let lints = parse_macro_input!(input as LintList).0;

        let mut lock = LINTS.lock().unwrap();
        if lints.is_empty() {
            Span::call_site()
                .warning("no lint levels are being defined in this macro call")
                .help("remove this macro call")
                .emit();
        } else {
            for lint in lints {
                lock.insert(lint, Level::Error);
            }
        }

        TokenStream::new()
    }
}

/// Sets the lint level of all listed lints to none.
///
/// Example:
/// ```no_run
/// json_proc::allow_json!(duplicate_keys);
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will not do ANYTHING
///     })
/// }
/// ```
#[proc_macro]
pub fn allow_json(input: TokenStream) -> TokenStream {
    lints::allow(input)
}

/// Sets the lint level of all listed lints to warning.
///
/// Example:
/// ```no_run
/// json_proc::warn_json!(duplicate_keys);
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will warn
///     })
/// }
/// ```
#[proc_macro]
pub fn warn_json(input: TokenStream) -> TokenStream {
    lints::warn(input)
}

/// Sets the lint level of all listed lints to error.
///
/// Example:
/// ```no_run
/// json_proc::error_json!(duplicate_keys);
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will error
///     })
/// }
/// ```
#[proc_macro]
pub fn error_json(input: TokenStream) -> TokenStream {
    lints::error(input)
}

/// Sets the lint level of all listed lints to error.
/// This attribute is an alias for [`error_json`].
///
/// Example:
/// ```no_run
/// json_proc::deny_json!(duplicate_keys);
/// fn test() {
///     json!({
///         "key": 2,
///         "key": 3 // duplicate, will error
///     })
/// }
/// ```
/// 
/// [`error_json`]: crate::error_json
#[proc_macro]
pub fn deny_json(input: TokenStream) -> TokenStream {
    error_json(input)
}

/// Derive the ToJson trait for a struct.
/// 
/// Examples:
/// 
/// ```no_run
/// #[derive(ToJson)]
/// struct Example1 {
///     field1: bool,
///     field2: i8
/// }
/// 
/// fn print() {
///     println!("{}", json!(Example1 {
///         field1: true,
///         field2: -2
///     }))
/// }
/// ```
#[proc_macro_derive(ToJson)]
// TODO: add enum support
pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let ident = input.ident;
    let mut members = input.fields.members().peekable();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if members.peek().is_some_and(|v| matches!(v, Member::Unnamed(_))) {
        if members.clone().count() == 1 {
            // Generate an impl that uses the first (and only) element in the tuple.
            quote! {
                impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                    fn to_json_string(&self) -> String {        
                        json_proc::ToJson::to_json_string(&self.0)
                    }
                }
            }
        } else {
            // Generate an array-like impl.
            quote! {
                impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                    fn to_json_string(&self) -> String {
                        let mut values: Vec<String> = Vec::new();
        
                        #(
                            values.push(json_proc::ToJson::to_json_string(&self.#members));
                        )*
        
                        format!("[{}]", values.into_iter().map(|val| format!("{val}")).collect::<Vec<String>>().join(","))
                    }
                }
            }
        }
    } else {
        // Generate an object-like impl.
        quote! {
            impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                fn to_json_string(&self) -> String {
                    let mut pairs: Vec<(String, String)> = Vec::new();
    
                    #(
                        let key = stringify!(#members);
                        let value = json_proc::ToJson::to_json_string(&self.#members);
                        pairs.push((key.to_string(), value));
                    )*
    
                    format!("{{{}}}", pairs.into_iter().map(|(key, val)| format!("\"{key}\":{val}")).collect::<Vec<String>>().join(","))
                }
            }
        }
    } .into()
}
