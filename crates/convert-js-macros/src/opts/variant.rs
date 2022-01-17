use darling::{util::Flag, FromVariant};

use crate::rename::{Rename, RenameRule};

use super::{check_fields, ConvertJsOptsStructData, FieldOptsInput};

#[derive(Debug, FromVariant)]
#[darling(attributes(convert_js))]
pub struct VariantOptsInput {
    ident: syn::Ident,
    discriminant: Option<syn::Expr>,
    fields: darling::ast::Fields<FieldOptsInput>,

    #[darling(default)]
    rename: Option<Rename>,

    /// Rename fields of this variant's content if this variant is a struct
    #[darling(default)]
    rename_all: Option<RenameRule>,
    /// Convert this variant's content as a single item tuple
    /// if it is a new type style struct
    #[darling(default)]
    new_type_as_tuple: Flag,
}

pub struct VariantOpts {
    pub ident: syn::Ident,
    pub data: ConvertJsOptsStructData,

    pub rename: Option<Rename>,
}

impl VariantOpts {
    pub fn try_from_input(
        input: VariantOptsInput,
        inherited_rename_all: Option<RenameRule>,
    ) -> Result<Self, String> {
        let mut v: Self = input.try_into()?;
        match &mut v.data {
            ConvertJsOptsStructData::Object { rename_all, .. } => {
                *rename_all = rename_all.or(inherited_rename_all);
            }
            _ => {}
        };

        Ok(v)
    }
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
            rename_all,
        } = value;

        if discriminant.is_some() {
            Err(format!(
                "macro ToJs doesn't support enum variant `{}` with discriminants",
                ident
            ))
        } else {
            Ok(Self {
                ident,
                data: check_fields(fields, new_type_as_tuple, rename_all)?,
                rename,
            })
        }
    }
}
