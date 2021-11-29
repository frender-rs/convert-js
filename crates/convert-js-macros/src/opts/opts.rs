use darling::{ast::Fields, util::Flag, FromDeriveInput};
use proc_macro2::Span;

use super::{FieldOptsInput, IndexedFieldOpts, NamedFieldOpts, NewTypeFieldOpts, VariantOptsInput};

#[derive(FromDeriveInput)]
#[darling(attributes(convert_js))]
pub struct ConvertJsOptsInput {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: darling::ast::Data<VariantOptsInput, FieldOptsInput>,

    #[darling(default)]
    pub rename_all: Option<crate::rename::RenameRule>,

    // enum style
    #[darling(default)]
    pub all_external: Flag,
    #[darling(default)]
    pub tag: Option<String>,
    #[darling(default)]
    pub content: Option<String>,
    #[darling(default)]
    pub untagged: Flag,
    #[darling(default)]
    pub union: Flag,

    // struct style
    #[darling(default)]
    pub new_type_as_tuple: Flag,
}

pub enum ConvertJsOptsData {
    Enum {
        variants: Vec<VariantOptsInput>,
        convert_style: super::EnumConvertJsStyle,
    },
    Struct {
        data: ConvertJsOptsStructData,
    },
}

pub enum ConvertJsOptsStructData {
    Unit,
    NewType(NewTypeFieldOpts),
    Tuple(Vec<IndexedFieldOpts>),
    Object(Vec<NamedFieldOpts>),
}

pub struct ConvertJsOpts {
    pub ident: syn::Ident,
    pub generics: syn::Generics,

    pub data: ConvertJsOptsData,

    pub rename_all: Option<crate::rename::RenameRule>,
}

impl TryFrom<ConvertJsOptsInput> for ConvertJsOpts {
    type Error = String;

    fn try_from(value: ConvertJsOptsInput) -> Result<Self, Self::Error> {
        let ConvertJsOptsInput {
            ident,
            generics,
            data,
            rename_all,

            all_external,
            content,
            tag,
            union,
            untagged,

            new_type_as_tuple,
        } = value;

        match data {
            darling::ast::Data::Enum(variants) => {
                crate::util::not_present!(new_type_as_tuple, enum)?;

                let convert_style = super::EnumConvertJsStyleInput {
                    all_external,
                    content,
                    tag,
                    union,
                    untagged,
                }
                .try_into()?;
                Ok(ConvertJsOpts {
                    ident,
                    generics,
                    data: ConvertJsOptsData::Enum {
                        variants,
                        convert_style,
                    },
                    rename_all,
                })
            }
            darling::ast::Data::Struct(fields) => {
                crate::util::not_present!(all_external, struct)?;
                crate::util::not_present!(content, struct)?;
                crate::util::not_present!(tag, struct)?;
                crate::util::not_present!(union, struct)?;
                crate::util::not_present!(untagged, struct)?;

                let data = super::check_fields(fields, new_type_as_tuple)?;

                Ok(ConvertJsOpts {
                    ident,
                    generics,
                    data: ConvertJsOptsData::Struct { data },
                    rename_all,
                })
            }
        }
    }
}
