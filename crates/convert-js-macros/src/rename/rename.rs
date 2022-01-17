use darling::FromMeta;
use syn::NestedMeta;

use super::RenameRule;

/// `#[convert_js(rename(rule = "lowercase"))]`
///
/// `#[convert_js(rename = "my_custom_name")]`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Rename {
    WithRule(RenameRule),
    Literal(String),
}

impl Rename {
    pub fn rename_field(&self, field_name: &str) -> String {
        match self {
            Rename::WithRule(rule) => rule.apply_to_field(field_name),
            Rename::Literal(name) => name.to_owned(),
        }
    }

    pub fn rename_variant(&self, variant_name: &str) -> String {
        match self {
            Rename::WithRule(rule) => rule.apply_to_variant(variant_name),
            Rename::Literal(name) => name.to_owned(),
        }
    }
}

impl FromMeta for Rename {
    /// `#[convert_js(rename(rule = "lowercase"))]`
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct WithRenameRule {
            rule: RenameRule,
        }

        WithRenameRule::from_list(items).map(|v| Rename::WithRule(v.rule))
    }

    /// `#[convert_js(rename = "my_custom_name")]`
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self::Literal(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use darling::{FromDeriveInput, FromField};
    use syn::parse_quote;

    use super::{super::RenameRule, Rename};

    #[test]
    fn test_rename_rule() {
        #[derive(FromDeriveInput)]
        #[darling(attributes(convert_js_test))]
        struct Opts {
            rename_all: RenameRule,
        }

        let input = parse_quote! {
            #[derive(ConvertJsTest)]
            #[convert_js_test(rename_all = "UPPERCASE")]
            struct Test;
        };

        let opts = Opts::from_derive_input(&input).unwrap();

        assert_eq!(opts.rename_all, RenameRule::UpperCase);
    }

    #[test]
    fn test_rename() {
        #[derive(FromField)]
        #[darling(attributes(convert_js_test))]
        struct FieldOpts {
            ident: Option<syn::Ident>,
            rename: Rename,
        }

        #[derive(FromDeriveInput)]
        #[darling(attributes(convert_js_test))]
        struct Opts {
            data: darling::ast::Data<(), FieldOpts>, // fields: Option<darling::ast::Fields<FieldOpts>>,
        }

        {
            let input = parse_quote! {
                #[derive(ToJsTest)]
                struct Test {
                    #[convert_js_test(rename(rule = "SCREAMING-KEBAB-CASE"))]
                    test_field: usize
                }
            };

            let opts = Opts::from_derive_input(&input).unwrap();

            let fields = opts.data.take_struct().unwrap().fields;

            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].ident.as_ref().unwrap().to_string(), "test_field");
            assert_eq!(
                fields[0].rename,
                Rename::WithRule(RenameRule::ScreamingKebabCase),
            );
        }

        {
            let input = parse_quote! {
                #[derive(ToJsTest)]
                struct Test (
                    #[convert_js_test(rename = "my_name")]
                    usize
                );
            };

            let opts = Opts::from_derive_input(&input).unwrap();

            let fields = opts.data.take_struct().unwrap().fields;

            assert_eq!(fields.len(), 1);
            assert!(fields[0].ident.is_none());
            assert_eq!(fields[0].rename, Rename::Literal("my_name".into()))
        }
    }
}
