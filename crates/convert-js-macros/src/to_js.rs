use std::collections::HashSet;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::opts::{self, ConvertJsOpts};

pub fn expand_derive_serialize(
    input: &mut syn::DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let opts = crate::opts::ConvertJsOptsInput::from_derive_input(&input);
    let opts = match opts {
        Ok(v) => v,
        Err(err) => return Ok(err.write_errors()),
    };

    let ConvertJsOpts {
        ident,
        generics,
        data,
    } = opts.try_into().unwrap();

    let impl_code = match data {
        crate::opts::ConvertJsOptsData::Enum {
            variants,
            convert_style,
            rename_all,
        } => opts::ImplEnum {
            ident: &ident,
            variants: &variants,
            convert_style: &convert_style,
            rename_all: rename_all.as_ref(),
        }
        .impl_to_js(),
        crate::opts::ConvertJsOptsData::Struct { data } => opts::ImplStruct {
            data: &data,
            ident: &ident,
            value_ident: None,
        }
        .impl_to_js(),
    }?;

    let (impl_generic_defs, impl_generic_names) =
        if let (Some(lt), Some(gt)) = (generics.lt_token, generics.gt_token) {
            let params = generics.params;

            let defs = params.iter().map(|gp| match gp {
                syn::GenericParam::Type(tp) => {
                    let mut tp = tp.clone();
                    let eq_token = tp.eq_token.take();
                    let default = tp.default.take();
                    // add ToJs to type param
                    if tp.colon_token.is_some() {
                        quote! {
                            #tp + ::convert_js::ToJs #eq_token #default
                        }
                    } else {
                        quote! {
                            #tp : ::convert_js::ToJs #eq_token #default
                        }
                    }
                }
                syn::GenericParam::Lifetime(lt) => lt.to_token_stream(),
                syn::GenericParam::Const(cp) => cp.to_token_stream(),
            });

            let defs = quote! { #lt #(#defs),* #gt };

            let names = params.into_iter().map(|gp| -> TokenStream {
                match gp {
                    syn::GenericParam::Type(ty) => ty.ident.to_token_stream(),
                    syn::GenericParam::Lifetime(lt) => lt.lifetime.to_token_stream(),
                    syn::GenericParam::Const(cp) => cp.ident.to_token_stream(),
                }
            });

            let names = quote! { #lt #(#names),* #gt };
            (Some(defs), Some(names))
        } else {
            (None, None)
        };

    let impl_where = generics.where_clause;

    let output = quote! {
        impl #impl_generic_defs ::convert_js::ToJs for #ident #impl_generic_names #impl_where {
            fn to_js(&self) -> ::convert_js::__internal::JsValue {
                #impl_code
            }
        }
    };

    Ok(output)
}
