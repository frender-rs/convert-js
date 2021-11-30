use darling::{ast::Fields, util::Flag, FromField};

use super::ConvertJsOptsStructData;
use crate::rename::{Rename, RenameRule};

#[derive(Debug, FromField)]
#[darling(attributes(convert_js))]
pub struct FieldOptsInput {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    rename: Option<Rename>,
}

pub struct NamedFieldOpts {
    pub ident: syn::Ident,
    pub ty: syn::Type,

    pub rename: Option<Rename>,
}

impl NamedFieldOpts {
    pub fn as_property_name(&self, rename_all: Option<&RenameRule>) -> String {
        let field_name = self.ident.to_string();
        if let Some(rename) = &self.rename {
            rename.rename_field(&field_name)
        } else if let Some(rename_all) = rename_all {
            rename_all.apply_to_field(&field_name)
        } else {
            field_name
        }
    }
}

pub struct IndexedFieldOpts {
    pub index: syn::Index,
    pub ty: syn::Type,
}

pub struct NewTypeFieldOpts {
    pub ty: syn::Type,
}

fn check_new_type_field(field: FieldOptsInput) -> Result<NewTypeFieldOpts, String> {
    match field {
        FieldOptsInput {
            ident: None,
            ty,
            rename: None,
        } => Ok(NewTypeFieldOpts {
            //
            ty,
        }),
        _ => Err(format!(
            "field in struct in NewType style cannot be named or renamed"
        )),
    }
}

fn check_named_fields(fields: Vec<FieldOptsInput>) -> Result<Vec<NamedFieldOpts>, String> {
    fields
        .into_iter()
        .map(
            |FieldOptsInput {
                 //
                 ident,
                 ty,
                 rename,
             }| match ident {
                Some(ident) => Ok(NamedFieldOpts {
                    //
                    ident,
                    ty,
                    rename,
                }),
                None => Err(
                    "struct with named fields must not contain fields without identifier".into(),
                ),
            },
        )
        .collect()
}

fn check_indexed_fields(fields: Vec<FieldOptsInput>) -> Result<Vec<IndexedFieldOpts>, String> {
    fields
        .into_iter()
        .enumerate()
        .map(
            |(
                i,
                FieldOptsInput {
                    ident,
                    ty,
                    rename: _,
                },
            )| match ident {
                Some(ident) => Err(format!(
                    "tuple struct must not contain named field `{}`",
                    ident
                )),
                None => Ok(IndexedFieldOpts {
                    ty,
                    index: syn::Index::from(i),
                }),
            },
        )
        .collect()
}

pub fn check_fields(
    fields: Fields<FieldOptsInput>,
    new_type_as_tuple: Flag,
) -> Result<ConvertJsOptsStructData, String> {
    let Fields { style, fields, .. } = fields;
    let data = match style {
        darling::ast::Style::Tuple => {
            if fields.len() == 1 {
                if new_type_as_tuple.is_some() {
                    let fields = check_indexed_fields(fields)?;
                    Ok(ConvertJsOptsStructData::Tuple(fields))
                } else {
                    let mut fields = fields;
                    let field = fields.pop().unwrap();
                    let opts = check_new_type_field(field)?;
                    Ok(ConvertJsOptsStructData::NewType(opts))
                }
            } else {
                crate::util::not_present!(new_type_as_tuple, "tuple struct with multiple fields")?;
                let fields = check_indexed_fields(fields)?;
                Ok(ConvertJsOptsStructData::Tuple(fields))
            }
        }
        darling::ast::Style::Struct => {
            let fields = check_named_fields(fields)?;
            Ok(ConvertJsOptsStructData::Object(fields))
        }
        darling::ast::Style::Unit => {
            if fields.len() == 0 {
                Ok(ConvertJsOptsStructData::Unit)
            } else {
                Err("unit struct cannot contain fields".to_string())
            }
        }
    }?;

    Ok(data)
}
