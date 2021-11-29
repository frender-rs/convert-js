use darling::{util::Flag, FromField, FromTypeParam, FromVariant};

use crate::rename::Rename;

#[derive(Debug, FromTypeParam)]
#[darling(attributes(convert_js))]
pub struct TypeParamOptsInput {
    pub ident: syn::Ident,
    pub bounds: Vec<syn::TypeParamBound>,

    #[darling(default)]
    pub ignore: Flag,
}
