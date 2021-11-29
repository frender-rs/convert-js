use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse_macro_input;

mod opts;
mod rename;
mod to_js;
mod util;

#[proc_macro_derive(ToJs, attributes(convert_js))]
pub fn derive_to_js(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input);

    to_js::expand_derive_serialize(&mut input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
