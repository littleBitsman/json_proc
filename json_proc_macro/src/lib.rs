#![cfg_attr(lints_enabled, feature(proc_macro_diagnostic))]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parse,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    token, Error as SynError, Expr, Index, Ident, ItemEnum, ItemStruct, LitBool, LitStr, Member,
    Result as SynResult, Token,
};

#[cfg(lints_enabled)]
// These only work on nightly because they are unstable
use proc_macro::{Diagnostic, Level};

mod util {
    use std::iter::Iterator;

    #[must_use]
    #[inline(always)]
    // This exists the way it does for two reasons:
    // 1. utility
    // 2. proc macros are expanded during build
    //    so as long as this isn't done at runtime
    //    it's more or less OK
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
    Null,
}

#[derive(Clone)]
enum JsonKey {
    Expr(Expr),
    Lit(String),
}

impl PartialEq for JsonKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lit(s1), Self::Lit(s2)) => *s1 == *s2,
            (Self::Expr(e1), Self::Expr(e2)) => quote!(#e1).to_string() == quote!(#e2).to_string(),
            _ => false,
        }
    }
}

struct JsonKeyValue {
    key: JsonKey,
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
        let (key, span) = if input.peek(LitStr) {
            let item = input.parse::<LitStr>()?;
            (JsonKey::Lit(item.value()), item.span())
        } else if cfg!(feature = "exprs-as-keys") {
            let item = input.parse::<Expr>()?;
            (JsonKey::Expr(item.clone()), item.span())
        } else {
            let item = input.parse::<Ident>()?;
            (JsonKey::Lit(item.to_string()), item.span())
        };
        input.parse::<Token![:]>()?;

        Ok(JsonKeyValue {
            key,
            value: input.parse()?,
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
            let pair = content.parse()?;
            pairs.push(pair);
            let _ = content.parse::<Token![,]>();
        }

        {
            let mut map: Vec<(JsonKey, Span)> = Vec::new();
            for JsonKeyValue { key, key_span, .. } in &pairs {
                #[cfg(lints_enabled)]
                if let Some((key, span2)) = map.iter().find(|(key2, _)| *key2 == *key) {
                    Diagnostic::spanned(
                        key_span.unwrap(),
                        Level::Error,
                        format!(
                            "duplicate key {} in object",
                            match key {
                                JsonKey::Lit(str) => str.clone(),
                                JsonKey::Expr(expr) => format!("expression `{}`", quote!(#expr)),
                            }
                        ),
                    )
                    .help("remove this repeated key")
                    .span_note(span2.unwrap(), "key first defined here")
                    .emit();
                    continue;
                }
                map.push((key.clone(), *key_span));
            }
        }

        Ok(JsonObject { pairs })
    }
}

impl Parse for JsonArray {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        let _brackets = bracketed!(content in input);
        let mut elements = Vec::new();

        while !content.is_empty() {
            let elem = content.parse()?;
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
            Ok(JsonValue::String(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(JsonValue::Bool(input.parse::<LitBool>()?.value))
        } else if input.peek(token::Brace) {
            Ok(JsonValue::Object(input.parse()?))
        } else if input.peek(token::Bracket) {
            Ok(JsonValue::Array(input.parse()?))
        } else if input
            .fork()
            .parse::<Ident>()
            .map(|v| v.to_string())
            .is_ok_and(|v| v == "undefined" || v == "null")
        {
            input.parse::<Ident>()?;
            Ok(JsonValue::Null)
        } else {
            Ok(JsonValue::Expr(input.parse()?))
        }
    }
}

impl ToTokens for JsonValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            JsonValue::Object(obj) => obj.to_tokens(tokens),
            JsonValue::Array(arr) => arr.to_tokens(tokens),
            JsonValue::String(litstr) => quote!(format!("\"{}\"", #litstr)).to_tokens(tokens),
            JsonValue::Bool(b) => (*b).to_tokens(tokens),
            JsonValue::Expr(expr) => {
                quote!(::json_proc::ToJson::to_json_string(&(#expr))).to_tokens(tokens);
            }
            JsonValue::Null => quote!("null").to_tokens(tokens),
        }
    }
}

impl ToTokens for JsonKey {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Lit(str) => quote!(#str).to_tokens(tokens),
            Self::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

// Implementing quote for JsonObject to generate valid Rust code
impl ToTokens for JsonObject {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.pairs.is_empty() {
            return quote!("{}".to_string()).to_tokens(tokens)
        }
        let pairs = &self.pairs;
        let mut pairs_tokens = Vec::new();
        for pair in pairs {
            let key = &pair.key;
            let value = &pair.value;
            pairs_tokens.push(quote!(format!("\"{}\":{}", #key, #value)));
        }
        let output = quote! {{
            // format!("{{{}}}", (vec![#(#pairs_tokens),*] as Vec<String>).join(","))
            let mut string = String::with_capacity(2);
            string.push('{');
            #(
                string.push_str(&#pairs_tokens);
                string.push(',');
            )*
            let _ = string.pop();
            string.push('}');
            string
        }};
        output.to_tokens(tokens);
    }
}

impl ToTokens for JsonArray {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.elements.is_empty() {
            return quote!("[]").to_tokens(tokens)
        }
        let elements = &self.elements;
        let mut elements_tokens = Vec::new();
        for elem in elements {
            elements_tokens.push(quote!(#elem));
        }
        let output = quote! {{
            // format!("[{}]", (vec![#(#elements_tokens),*] as Vec<String>).join(","))
            let mut string = String::with_capacity(2);
            string.push('[');
            #(
                string.push_str(&#elements_tokens);
                string.push(',');
            )*
            let _ = string.pop();
            string.push(']');
            string
        }};
        output.to_tokens(tokens);
    }
}

// Lints only happen on nightly since diagnostics are
// unstable features, so change the docs to reflect that.
#[cfg_attr(lints_enabled, doc = "Lints and properly ")]
#[cfg_attr(not(lints_enabled), doc = "Properly ")]
/// formats a JSON object, array, or value.
///
/// This macro supports:
/// - all literals (integers, floats, [`&str`][strlit], [`char`])
/// - any expression that evaluates to a [`impl ToJson`][ToJson]
///
/// If you are looking for custom serialization traits, macros,
/// and functions, use `serde_json` and `serde` instead.
/// 
#[cfg_attr(not(lints_enabled), doc = "Keep in mind, lints are only enabled on the Nightly channel of Rust.")]
/// ## Examples:
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
/// [strlit]: str
/// [ToJson]: https://docs.rs/json_proc/latest/json_proc/trait.ToJson.html
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let json_value = parse_macro_input!(input as JsonValue);

    quote!(#json_value).into()
}

/// Derive the ToJson trait for a struct or enum.
///
/// ## Example:
///
/// ```no_run
/// # extern crate json_proc;
/// use json_proc::{ToJson, json};
/// 
/// #[derive(ToJson)]
/// struct Example1 {
///     field1: bool,
///     field2: i8
/// }
///
/// # #[test]
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
        let ident = &input.ident;
        let mut members = input.fields.members().peekable();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let type_generics = input.generics.type_params().map(|v| v.ident.clone());

        let where_clause = where_clause.map_or_else(
            || {
                quote! {
                    where
                        #(
                            #type_generics: ::json_proc::ToJson
                        ),*
                }
            },
            |v| quote!(#v),
        );

        let fn_impl = if members
            .peek()
            .is_some_and(|v| matches!(v, Member::Unnamed(_)))
        {
            if util::iter_len(&members) == 1 {
                // Generate an impl that uses the first (and only) element in the tuple.
                quote!(::json_proc::ToJson::to_json_string(&self.0))
            } else {
                // Generate an array-like impl.
                quote! {{
                    // format!("[{}]", (vec![#(#elements_tokens),*] as Vec<String>).join(","))
                    let mut string = String::with_capacity(2);
                    string.push('[');
                    #(
                        string.push_str(&(::json_proc::ToJson::to_json_string(&self.#members)));
                        string.push(',');
                    )*
                    let _ = string.pop();
                    string.push(']');
                    string
                }}
            }
        } else if members.peek().is_some() {
            // Generate an object-like impl.
            quote! {{
                // format!("{{{}}}", (vec![#(#pairs_tokens),*] as Vec<String>).join(","))
                let mut string = String::with_capacity(2);
                string.push('{');
                #(
                    string.push('"');
                    string.push_str(stringify!(#members));
                    string.push('"');
                    string.push(':');
                    string.push_str(&(::json_proc::ToJson::to_json_string(&self.#members)));
                    string.push(',');
                )*
                let _ = string.pop();
                string.push('}');
                string
            }}
        } else {
            quote!(stringify!(#ident).to_string())
        };
        quote! {
            impl #impl_generics ToJson for #ident #ty_generics #where_clause {
                fn to_json_string(&self) -> String {
                    #fn_impl
                }
            }
        }
        .into()
    } else if let Ok(input) = parse::<ItemEnum>(item.clone()) {
        let ident = input.ident;
        let variants = input.variants.iter().peekable();
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let type_generics = input.generics.type_params().map(|v| v.ident.clone());

        let where_clause = where_clause.map_or_else(
            || {
                quote! {
                    where
                        #(
                            #type_generics: ::json_proc::ToJson
                        ),*
                }
            },
            |v| quote!(#v),
        );

        let mut streams: Vec<TokenStream2> = Vec::new();
        for var in variants {
            // Handle like a struct.
            let varident = &var.ident;
            let mut members = var.fields.members().peekable();
            let iter_len = util::iter_len(&members);
            let this_impl = if iter_len == 0 {
                quote!(Self::#varident => stringify!(#ident).to_string())
            } else if members
                .peek()
                .is_some_and(|v| matches!(v, Member::Unnamed(_)))
            {
                if iter_len == 1 {
                    // Generate an impl that uses the first (and only) element in the tuple.
                    quote!(Self::#varident(a) => ::json_proc::ToJson::to_json_string(a))
                } else {
                    // Generate an array-like impl.
                    let members = members.map(|v| match v {
                        Member::Unnamed(i) => {
                            Ident::new(format!("arg{}", i.index).as_str(), i.span)
                        }
                        Member::Named(_) => unreachable!(),
                    });
                    let members2 = members.clone();
                    quote!(Self::#varident( #(#members2),* ) => {
                        // format!("[{}]", (vec![#(#elements_tokens),*] as Vec<String>).join(","))
                        let mut string = String::with_capacity(2);
                        string.push('[');
                        #(
                            string.push_str(&(::json_proc::ToJson::to_json_string(#members)));
                            string.push(',');
                        )*
                        let _ = string.pop();
                        string.push(']');
                        string
                    })
                }
            } else {
                // Generate an object-like impl.
                let members2 = members.clone();
                quote!(Self::#varident { #(#members2),* } => {
                    // format!("{{{}}}", (vec![#(#pairs_tokens),*] as Vec<String>).join(","))
                    let mut string = String::with_capacity(2);
                    string.push('{');
                    #(
                        string.push('"');
                        string.push_str(stringify!(#members));
                        string.push('"');
                        string.push(':');
                        string.push_str(&(::json_proc::ToJson::to_json_string(#members)));
                        string.push(',');
                    )*
                    let _ = string.pop();
                    string.push('}');
                    string
                })
            };

            streams.push(this_impl)
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
        SynError::new(
            TokenStream2::from(item).span(),
            "expected struct or enum for deriving ToJson",
        )
        .into_compile_error()
        .into()
    }
}

#[doc(hidden)]
#[proc_macro]
/// Private macro for generating impls of ToJson for tuples.
pub fn tuple_impl(_: TokenStream) -> TokenStream {
    const LETTERS: [char; 12] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L'];
    let mut streams = Vec::with_capacity(12);
    for i in 0..12 {
        let letters = &LETTERS[..(i + 1)].iter().map(|ch| Ident::new(&ch.to_string(), Span::call_site())).collect::<Vec<Ident>>();
        let nums = (0..(i + 1)).map(Index::from).collect::<Vec<Index>>();
        let doc_attr = if i == 0 {
            quote!(#[doc = "`ToJson` is implemented for tuples up to size 12."])
        } else {
            quote!(#[doc(hidden)])
        };
        streams.push(quote! {
            #doc_attr
            impl<#(#letters),*> crate::ToJson for (#(#letters,)*) 
            where
                #(
                    #letters: crate::ToJson
                ),*
            {
                fn to_json_string(&self) -> String {
                    let mut string = String::with_capacity(3);
                    string.push('[');
                    #(
                        string.push_str(&(crate::ToJson::to_json_string(&self.#nums)));
                        string.push(',');
                    )*
                    let _ = string.pop();
                    string.push(']');
                    string
                } 
            }
        });
    }
    quote!(#(#streams)*).into()
}
