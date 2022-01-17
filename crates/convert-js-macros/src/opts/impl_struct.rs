use std::borrow::Cow;

use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};

use super::ConvertJsOptsStructData;

pub struct ImplStruct<'a> {
    pub ident: &'a syn::Ident,
    pub data: &'a ConvertJsOptsStructData,
    /// default to `self`
    pub value_ident: Option<&'a syn::Ident>,
}

impl<'a> ImplStruct<'a> {
    pub fn impl_to_js(&self) -> Result<TokenStream, Vec<syn::Error>> {
        let Self {
            ident,
            data,
            value_ident,
        } = *self;

        let value_ident = if let Some(value_ident) = value_ident {
            Cow::Borrowed(value_ident)
        } else {
            Cow::Owned(syn::Ident::new("self", Span::call_site()))
        };

        let ts = match data {
            crate::opts::ConvertJsOptsStructData::Unit => {
                let span = ident.span();
                return Err(vec![syn::Error::new(
                    span,
                    "macro ToJs doesn't support unit struct currently",
                )]);
            }
            crate::opts::ConvertJsOptsStructData::NewType(field) => {
                let ty = &field.ty;
                quote! {
                    <#ty as ::convert_js::ToJs>::to_js(&#value_ident.0)
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
                            let i = &field.index;
                            let ty = &field.ty;
                            quote! { &<#ty as ::convert_js::ToJs>::to_js(&#value_ident.#i) }
                        }),
                        quote! {,},
                    );

                    if len <= 5 {
                        let method = proc_macro2::Ident::new(
                            &format!("of{}", len),
                            proc_macro2::Span::call_site(),
                        );
                        quote! { ::convert_js::js_sys::Array::#method( #list ).into() }
                    } else {
                        quote! { ::convert_js::js_sys::Array::from_iter([ #list ]).into() }
                    }
                }
            }
            crate::opts::ConvertJsOptsStructData::Object { fields, rename_all } => {
                let mut tokens = quote! {
                    ::convert_js::__internal::JsObject::new()
                };

                tokens.append_all(fields.into_iter().map(|field| {
                    let prop = field.as_property_name(rename_all.as_ref());
                    let prop = proc_macro2::Literal::string(&prop);

                    let ty = &field.ty;
                    let field = &field.ident;

                    quote! { .with_prop( &#prop , &<#ty as ::convert_js::ToJs>::to_js(&#value_ident.#field) ) }
                }));

                quote! { #tokens.into_js_value() }
            }
        };

        Ok(ts)
    }
}
