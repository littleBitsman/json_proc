#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, Level, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    braced, bracketed, parse,
    parse::{Parse, ParseStream, Result as SynResult},
    parse_macro_input, token, Expr, Ident, ItemEnum, ItemStruct, LitBool, LitStr, Member, Token,
};

mod util {
    use std::iter::Iterator;

    pub fn iter_len<T, I: Iterator<Item = T> + Clone>(iter: &I) -> usize {
        iter.clone().count()
    }
}

enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(LitStr),
    Bool(bool),
    Expr(Expr),
}

struct JsonKeyValue {
    key: String,
    value: JsonValue,
    key_span: Span,
}

struct JsonObject {
    pairs: Vec<JsonKeyValue>,
}

struct JsonArray {
    elements: Vec<JsonValue>,
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
            let mut map: Vec<(String, Span)> = Vec::new();
            for JsonKeyValue { key, key_span, .. } in pairs.iter() {
                if let Some((_, span2)) = 
                    map.iter().find(|(key2, _)| key2.clone() == key.clone()) 
                {
                    let mut d = Diagnostic::new(Level::Error, format!("duplicate key `{key}` in object"));
                    d.set_spans(key_span.unwrap());
                    d = d.span_note(span2.unwrap(), "key first defined here");
                    d.emit();
                } else {
                    map.push((key.clone(), *key_span));
                }
            }
        }

        Ok(JsonObject {
            pairs,
            // span: input.span(),
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
            // span: input.span(),
        })
    }
}

/* // This is never used
impl JsonValue {
    fn span(&self) -> Option<Span> {
        match self {
            JsonValue::Object(json_object) => Some(json_object.span.clone()),
            JsonValue::Array(json_array) => Some(json_array.span),
            JsonValue::String(lit_str) => Some(lit_str.span()),
            JsonValue::Number(expr) => Some(expr.span()),
            JsonValue::Bool(_) => None,
            JsonValue::Expr(expr) => Some(expr.span())
        }
    }
}
*/

impl Parse for JsonValue {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(LitStr) {
            let s: LitStr = input.parse()?;
            Ok(JsonValue::String(s))
        } else if input.peek(LitBool) {
            Ok(JsonValue::Bool(input.parse::<LitBool>()?.value))
        } else if input.peek(token::Brace) {
            let obj: JsonObject = input.parse()?;
            Ok(JsonValue::Object(obj))
        } else if input.peek(token::Bracket) {
            let arr: JsonArray = input.parse()?;
            Ok(JsonValue::Array(arr))
        } else {
            let expr: Expr = input.parse()?;
            Ok(JsonValue::Expr(expr))
        }
    }
}

impl quote::ToTokens for JsonValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            JsonValue::Object(obj) => obj.to_tokens(tokens),
            JsonValue::Array(arr) => arr.to_tokens(tokens),
            JsonValue::String(litstr) => quote!(format!("\"{}\"", #litstr)).to_tokens(tokens),
            JsonValue::Bool(b) => {
                let b = *b;
                let token = quote!(#b);
                token.to_tokens(tokens);
            }
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
        let output = quote!(format!(
            "[{}]",
            vec![#(#elements_tokens.to_string()),*].join(",")
        ));
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
pub fn json_derive(item: TokenStream) -> TokenStream {
    if let Ok(input) = parse::<ItemStruct>(item.clone()) {
        let ident = input.ident;
        let mut members = input.fields.members().peekable();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let fn_impl = if members
            .peek()
            .is_some_and(|v| matches!(v, Member::Unnamed(_)))
        {
            if util::iter_len(&members) == 1 {
                // Generate an impl that uses the first (and only) element in the tuple.
                quote!(json_proc::ToJson::to_json_string(&self.0))
            } else {
                // Generate an array-like impl.
                quote! {
                    let values: Vec<String> = vec![#(json_proc::ToJson::to_json_string(&self.#members)),*];

                    format!("[{}]", values.into_iter().map(|val| format!("{val}")).collect::<Vec<String>>().join(","))
                }
            }
        } else {
            // Generate an object-like impl.
            quote! {
                let mut pairs: Vec<(String, String)> = Vec::new();

                #({
                    let key = stringify!(#members);
                    let value = json_proc::ToJson::to_json_string(&self.#members);
                    pairs.push((key.to_string(), value));
                })*

                format!("{{{}}}", pairs.into_iter().map(|(key, val)| format!("\"{key}\":{val}")).collect::<Vec<String>>().join(","))
            }
        };

        quote! {
            impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                fn to_json_string(&self) -> String {
                    #fn_impl
                }
            }
        }
        .into()
    } else if let Ok(input) = parse::<ItemEnum>(item) {
        let ident = input.ident;
        let variants = input.variants.iter().peekable();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let mut streams: Vec<TokenStream2> = Vec::new();
        for var in variants {
            // Handle like a struct.
            let varident = &var.ident;
            let mut members = var.fields.members().peekable();
            let iter_len = util::iter_len(&members);
            let str = if iter_len == 0 {
                quote!(Self::#varident => stringify!(#ident).to_string())
            } else if members
                .peek()
                .is_some_and(|v| matches!(v, Member::Unnamed(_)))
            {
                if iter_len == 1 {
                    // Generate an impl that uses the first (and only) element in the tuple.
                    quote!(Self::#varident(a) => json_proc::ToJson::to_json_string(a))
                } else {
                    // Generate an array-like impl.
                    let members = members.map(|v| match v {
                        Member::Unnamed(i) => {
                            Ident::new(format!("arg{}", i.index).as_str(), i.span)
                        }
                        _ => unreachable!(),
                    });
                    let members2 = members.clone();
                    quote!(Self::#varident( #(#members2),* ) => {
                        let values: Vec<String> = vec![#(json_proc::ToJson::to_json_string(#members)),*];

                        format!("[{}]", values.into_iter().map(|val| format!("{val}")).collect::<Vec<String>>().join(","))
                    })
                }
            } else {
                // Generate an object-like impl.
                let members2 = members.clone();
                quote!(Self::#varident { #(#members2),* } => {
                    let mut pairs: Vec<(String, String)> = Vec::new();

                    #({
                        let key = stringify!(#members);
                        let value = json_proc::ToJson::to_json_string(#members);
                        pairs.push((key.to_string(), value));
                    })*

                    format!("{{{}}}", pairs.into_iter().map(|(key, val)| format!("\"{key}\":{val}")).collect::<Vec<String>>().join(","))
                })
            };

            streams.push(str)
        }

        quote! {
            impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                fn to_json_string(&self) -> String {
                    match self {
                        #(#streams),*
                    }
                }
            }
        }
        .into()
    } else {
        quote!(compile_error!(
            "expected struct or enum for deriving ToJson"
        ))
        .into()
    }
}
