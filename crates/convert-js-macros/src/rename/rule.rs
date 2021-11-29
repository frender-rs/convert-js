use darling::FromMeta;

/// Corresponding to [`serde(rename_all)`](https://serde.rs/container-attrs.html#rename_all)
///
/// [serde source code](https://github.com/serde-rs/serde/blob/b7bad3a1650c7232d559a9ba1efa0ff504989f57/serde_derive/src/internals/case.rs#L12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromMeta)]
pub enum RenameRule {
    /// Rename direct children to "lowercase" style.
    #[darling(rename = "lowercase")]
    LowerCase,
    /// Rename direct children to "UPPERCASE" style.
    #[darling(rename = "UPPERCASE")]
    UpperCase,
    /// Rename direct children to "PascalCase" style, as typically used for
    /// enum variants.
    #[darling(rename = "PascalCase")]
    PascalCase,
    /// Rename direct children to "camelCase" style.
    #[darling(rename = "camelCase")]
    CamelCase,
    /// Rename direct children to "snake_case" style, as commonly used for
    /// fields.
    #[darling(rename = "snake_case")]
    SnakeCase,
    /// Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
    /// used for constants.
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnakeCase,
    /// Rename direct children to "kebab-case" style.
    #[darling(rename = "kebab-case")]
    KebabCase,
    /// Rename direct children to "SCREAMING-KEBAB-CASE" style.
    #[darling(rename = "SCREAMING-KEBAB-CASE")]
    ScreamingKebabCase,
}

/// See [serde](https://github.com/serde-rs/serde/blob/3859f58d9b2624f7f0ca32e3b4a6557fea94bdee/serde_derive_internals/src/case.rs#L62)
impl RenameRule {
    /// Apply a renaming rule to an enum variant, returning the version expected in the source.
    pub fn apply_to_variant(&self, variant: &str) -> String {
        use RenameRule::*;
        match self {
            PascalCase => variant.to_owned(),
            LowerCase => variant.to_ascii_lowercase(),
            UpperCase => variant.to_ascii_uppercase(),
            CamelCase => variant[..1].to_ascii_lowercase() + &variant[1..],
            SnakeCase => {
                let mut snake = String::new();
                for (i, ch) in variant.char_indices() {
                    if i > 0 && ch.is_uppercase() {
                        snake.push('_');
                    }
                    snake.push(ch.to_ascii_lowercase());
                }
                snake
            }
            ScreamingSnakeCase => SnakeCase.apply_to_variant(variant).to_ascii_uppercase(),
            KebabCase => SnakeCase.apply_to_variant(variant).replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase
                .apply_to_variant(variant)
                .replace('_', "-"),
        }
    }

    /// Apply a renaming rule to a struct field, returning the version expected in the source.
    pub fn apply_to_field(&self, field: &str) -> String {
        use RenameRule::*;
        match self {
            LowerCase | SnakeCase => field.to_owned(),
            UpperCase => field.to_ascii_uppercase(),
            PascalCase => {
                let mut pascal = String::new();
                let mut capitalize = true;
                for ch in field.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            CamelCase => {
                let pascal = PascalCase.apply_to_field(field);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            ScreamingSnakeCase => field.to_ascii_uppercase(),
            KebabCase => field.replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase.apply_to_field(field).replace('_', "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use darling::FromDeriveInput;
    use syn::parse_quote;

    use super::RenameRule;

    #[test]
    fn parse_rename_rule() {
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
}
