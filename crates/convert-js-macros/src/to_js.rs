use std::str::FromStr;

use darling::{ast::Fields, FromDeriveInput, FromMeta};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::Token;

use crate::opts::ConvertJsOpts;

pub fn expand_derive_serialize(
    input: &mut syn::DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let opts = crate::opts::ConvertJsOptsInput::from_derive_input(&input).unwrap();
    let ConvertJsOpts {
        ident,
        generics,
        data,
        rename_all,
    } = opts.try_into().unwrap();

    let impl_code = match data {
        crate::opts::ConvertJsOptsData::Enum {
            variants,
            convert_style,
        } => {
            todo!()
        }
        crate::opts::ConvertJsOptsData::Struct { data } => match data {
            crate::opts::ConvertJsOptsStructData::Unit => {
                let span = ident.span();
                return Err(vec![syn::Error::new(
                    span,
                    "macro ToJs doesn't support unit struct currently",
                )]);
            }
            crate::opts::ConvertJsOptsStructData::NewType(field) => {
                quote! {
                    ::convert_js::ToJs::to_js(&self.0)
                }
            }
            crate::opts::ConvertJsOptsStructData::Tuple(fields) => {
                let len = fields.len();
                if len == 0 {
                    quote! { ::convert_js::js_sys::Array::new().into() }
                } else {
                    let mut list = TokenStream::new();
                    list.append_separated(
                        fields.into_iter().map(|field| {
                            let i = field.index;
                            quote! { &::convert_js::ToJs::to_js(&self.#i) }
                        }),
                        quote! {,},
                    );

                    if len <= 5 {
                        let method =
                            proc_macro2::Ident::from_string(&format!("of{}", len)).unwrap();
                        quote! { ::convert_js::js_sys::Array::#method( #list ).into() }
                    } else {
                        quote! { ::convert_js::js_sys::Array::from_iter([ #list ]).into() }
                    }
                }
            }
            crate::opts::ConvertJsOptsStructData::Object(fields) => {
                let mut tokens = quote! {
                    ::convert_js::__internal::JsObject::new()
                };

                tokens.append_all(fields.into_iter().map(|field| {
                    let prop = field.as_property_name();
                    let prop = proc_macro2::Literal::string(&prop);

                    let field = field.ident;

                    quote! { .with_prop( &#prop , &::convert_js::ToJs::to_js(&self.#field) ) }
                }));

                quote! { #tokens.into_js_value() }
            }
        },
    };

    let impl_type_params = if let (Some(lt), Some(gt)) = (generics.lt_token, generics.gt_token) {
        let params = generics.params;
        Some(quote! { #lt #params #gt })
    } else {
        None
    };

    let impl_where = generics.where_clause;

    let output = quote! {
        impl #impl_type_params ::convert_js::ToJs for #ident #impl_where {
            fn to_js(&self) -> ::convert_js::__internal::JsValue {
                #impl_code
            }
        }
    };

    Ok(output)
}
