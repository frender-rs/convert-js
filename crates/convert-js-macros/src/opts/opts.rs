use darling::{util::Flag, FromDeriveInput};

use super::{
    FieldOptsInput, IndexedFieldOpts, NamedFieldOpts, NewTypeFieldOpts, VariantOpts,
    VariantOptsInput,
};

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
        variants: Vec<VariantOpts>,
        convert_style: super::EnumConvertJsStyle,
        rename_all: Option<crate::rename::RenameRule>,
    },
    Struct {
        data: ConvertJsOptsStructData,
    },
}

#[derive(Debug, Clone)]
pub enum ConvertJsOptsStructData {
    Unit,
    NewType(NewTypeFieldOpts),
    Tuple(Vec<IndexedFieldOpts>),
    Object {
        fields: Vec<NamedFieldOpts>,
        rename_all: Option<crate::rename::RenameRule>,
    },
}

pub struct ConvertJsOpts {
    pub ident: syn::Ident,
    pub generics: syn::Generics,

    pub data: ConvertJsOptsData,
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

                let variants = variants
                    .into_iter()
                    .map(|var| VariantOpts::try_from_input(var, rename_all))
                    .collect::<Result<_, _>>()?;

                Ok(ConvertJsOpts {
                    ident,
                    generics,
                    data: ConvertJsOptsData::Enum {
                        variants,
                        convert_style,
                        rename_all,
                    },
                })
            }
            darling::ast::Data::Struct(fields) => {
                crate::util::not_present!(all_external, struct)?;
                crate::util::not_present!(content, struct)?;
                crate::util::not_present!(tag, struct)?;
                crate::util::not_present!(union, struct)?;
                crate::util::not_present!(untagged, struct)?;

                let data = super::check_fields(fields, new_type_as_tuple, rename_all)?;

                Ok(ConvertJsOpts {
                    ident,
                    generics,
                    data: ConvertJsOptsData::Struct { data },
                })
            }
        }
    }
}
