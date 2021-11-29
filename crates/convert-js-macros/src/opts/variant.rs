use darling::{util::Flag, FromVariant};

use crate::rename::Rename;

use super::{check_fields, ConvertJsOptsStructData, FieldOptsInput};

#[derive(Debug, FromVariant)]
#[darling(attributes(convert_js))]
pub struct VariantOptsInput {
    ident: syn::Ident,
    discriminant: Option<syn::Expr>,
    fields: darling::ast::Fields<FieldOptsInput>,

    #[darling(default)]
    rename: Option<Rename>,
    #[darling(default)]
    new_type_as_tuple: Flag,
}

pub struct VariantOpts {
    pub ident: syn::Ident,
    pub data: ConvertJsOptsStructData,

    pub rename: Option<Rename>,
}

impl TryFrom<VariantOptsInput> for VariantOpts {
    type Error = String;

    fn try_from(value: VariantOptsInput) -> Result<Self, Self::Error> {
        let VariantOptsInput {
            ident,
            discriminant,
            fields,
            rename,
            new_type_as_tuple,
        } = value;

        if discriminant.is_some() {
            Err(format!(
                "macro ToJs doesn't support enum variant `{}` with discriminants",
                ident
            ))
        } else {
            Ok(Self {
                ident,
                data: check_fields(fields, new_type_as_tuple)?,
                rename,
            })
        }
    }
}
