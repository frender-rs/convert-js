use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;

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
