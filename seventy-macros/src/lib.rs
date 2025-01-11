//! Procedural macros for Seventy.

use proc_macro::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, Error, ItemStruct, Meta, Token};

mod seventy;

#[proc_macro_attribute]
pub fn seventy(metas: TokenStream, item: TokenStream) -> TokenStream {
    let metas = match Punctuated::<Meta, Token![,]>::parse_terminated.parse(metas) {
        Ok(metas) => metas,
        Err(error) => return error.into_compile_error().into(),
    };

    let item = match syn::parse::<ItemStruct>(item) {
        Ok(item) => item,
        Err(error) => return error.into_compile_error().into(),
    };

    seventy::expand(metas, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
