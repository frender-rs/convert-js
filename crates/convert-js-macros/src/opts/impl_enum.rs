use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use super::{EnumConvertJsStyle, VariantOpts};
use crate::{opts, rename::RenameRule};

pub struct ImplEnum<'a> {
    pub ident: &'a syn::Ident,
    pub variants: &'a Vec<VariantOpts>,
    pub convert_style: &'a EnumConvertJsStyle,
    pub rename_all: Option<&'a RenameRule>,
}

impl<'a> ImplEnum<'a> {
    pub fn impl_to_js(&self) -> Result<TokenStream, Vec<syn::Error>> {
        let Self {
            ident: enum_ident,
            convert_style,
            variants,
            rename_all: enum_rename_all,
        } = *self;

        if variants.len() == 0 {
            return Err(vec![syn::Error::new(
                enum_ident.span(),
                "macro ToJs currently doesn't support empty enums.",
            )]);
        }

        let mut match_arms = TokenStream::new();
        match convert_style {
            EnumConvertJsStyle::Union => {
                match_arms.append_separated(
                    variants
                        .into_iter()
                        .map(|variant| -> Result<TokenStream, Vec<syn::Error>> {
                            let VariantOpts {
                                ident,
                                data,
                                rename,
                            } = &variant;

                            let value_ident =
                                syn::Ident::new("__value_of_enum_variant", Span::call_site());

                            let tag = if let Some(rename) = rename {
                                rename.rename_variant(&ident.to_string())
                            } else if let Some(enum_rename_all) = enum_rename_all {
                                enum_rename_all.apply_to_variant(&ident.to_string())
                            } else {
                                ident.to_string()
                            };
                            let tag = syn::LitStr::new(&tag, ident.span());

                            let impl_code = match data {
                                crate::opts::ConvertJsOptsStructData::Unit => {
                                    quote! {
                                        ::convert_js::__internal::JsValue::from_str(#tag)
                                    }
                                }
                                _ => opts::ImplStruct {
                                    data,
                                    ident,
                                    value_ident: Some(&value_ident),
                                }
                                .impl_to_js()?,
                            };

                            Ok(enum_variant_impl_to_js(&variant, &value_ident, impl_code))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    quote! {,},
                );
            }
            _ => {
                todo!()
            }
        };

        Ok(quote! {
            match self {
                #match_arms
            }
        })
    }
}

fn enum_variant_impl_to_js(
    variant: &VariantOpts,
    value_ident: &syn::Ident,
    impl_code: TokenStream,
) -> TokenStream {
    let VariantOpts { ident, data, .. } = variant;

    match data {
        opts::ConvertJsOptsStructData::Unit => quote! { Self::#ident => { #impl_code } },
        opts::ConvertJsOptsStructData::NewType(_) => {
            quote! {
                Self::#ident(#value_ident) => {
                    let #value_ident = (#value_ident,);
                    #impl_code
                }
            }
        }
        opts::ConvertJsOptsStructData::Tuple(fields) => {
            let tuple_content = indexed_fields_to_tuple_content(&fields, value_ident);
            quote! {
                Self::#ident(#tuple_content) => {
                    let #value_ident = (#tuple_content,);
                    #impl_code
                }
            }
        }
        opts::ConvertJsOptsStructData::Object { fields, rename_all } => {
            let struct_ident = format_ident!("Object__{}", value_ident);

            let struct_content = {
                let mut list = TokenStream::new();
                list.append_separated(fields.iter().map(|field| &field.ident), quote! {,});
                list
            };

            let struct_def = {
                let mut list = TokenStream::new();
                list.append_separated(
                    fields.iter().map(|field| {
                        let ident = &field.ident;
                        let ty = &field.ty;
                        quote! { #ident : &'a #ty }
                    }),
                    quote! {,},
                );
                list
            };

            quote! {
                Self::#ident { #struct_content } => {
                    struct #struct_ident<'a> {
                        #struct_def
                    }
                    let #value_ident = #struct_ident { #struct_content };

                    #impl_code
                }
            }
        }
    }
}

fn indexed_fields_to_tuple_content(
    fields: &Vec<opts::IndexedFieldOpts>,
    value_ident_prefix: &syn::Ident,
) -> TokenStream {
    let mut list = TokenStream::new();
    list.append_separated(
        fields
            .iter()
            .map(|field| format_ident!("{}__{}", value_ident_prefix, field.index)),
        quote! {,},
    );

    list
}
