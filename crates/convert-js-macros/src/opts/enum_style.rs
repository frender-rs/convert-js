use darling::util::Flag;

/// Please go to docs of each variant to see
/// what the following enum will be converted as.
///
/// ```compile_fail
/// # use convert_js::{ToJs, Nullable, Maybe };
///
/// #[derive(ToJs)]
/// enum MyEnum {
///     VariantNewType(String),
///     VariantTuple(f64, bool),
///     VariantStruct {
///         field1: Option<usize>,
///         field2: Maybe<usize>,
///         field3: Nullable<usize>,
///     },
///     VariantUnit,
/// }
/// ```
pub enum EnumConvertJsStyle {
    /// The default.
    ///
    /// The most explicit and also efficient style.
    ///
    /// `VariantUnit` will be converted as a string literal of its variant name.
    /// Other types of variants will be converted as `{ [variantName]: variantContent }`
    ///
    /// ```typescript
    /// | { VariantNewType: string }
    /// | { VariantTuple: [number, boolean] }
    /// | { VariantStruct: { field1?: number, field2: number | undefined, field3: number | null } }
    /// | "VariantUnit"
    /// ```
    External,

    /// `#[convert_js(all_external)]`
    ///
    /// Similar to [`External`](Self::External) except that
    /// `VariantUnit` will be converted as `{ [variantName]: undefined }`.
    ///
    /// ```typescript
    /// | { VariantNewType: string }
    /// | { VariantTuple: [number, boolean] }
    /// | { VariantStruct: { field1?: number, field2: number | undefined, field3: number | null } }
    /// | { VariantUnit: undefined }
    /// ```
    ///
    /// You can control the content of `VariantUnit` with `content_as`:
    ///
    /// ```compile_fail
    /// # use convert_js::ToJs;
    ///
    /// #[derive(ToJs)]
    /// enum MyEnum {
    ///     VariantNewType(String),
    ///     #[convert_js(content_as = true)]
    ///     VariantUnit,
    /// }
    /// ```
    ///
    /// ```typescript
    /// | { VariantNewType: string }
    /// | { VariantUnit: true }
    /// ```
    AllExternal,

    /// `#[convert_js(tag = "type")]`
    ///
    /// `VariantNewType` and `VariantTuple` are not allowed in this style.
    ///
    /// ```typescript
    /// | { type: "VariantStruct", field1?: number, field2: number | undefined, field3: number | null }
    /// | { type: "VariantUnit" }
    /// ```
    Internal { tag: String },

    /// `#[convert_js(tag = "t", content = "c")]`
    ///
    /// ```typescript
    /// | { t: "VariantNewType", c: string }
    /// | { t: "VariantTuple", c: [number, boolean] }
    /// | { t: "VariantStruct", c: { field1?: number, field2: number | undefined, field3: number | null } }
    /// | { t: "VariantUnit", c: never }
    /// ```
    Adjacent { tag: String, content: String },

    /// `#[convert_js(untagged)]`
    ///
    /// **Note that `VariantUnit` will be converted as `undefined`.
    /// When there are multiple `VariantUnit` variants, a compiling error is emitted.**
    ///
    /// You can use `#[convert_js(untagged, content_as = some_expr)]` to convert it as `some_expr`.
    /// In such situation, multiple `VariantUnit` variants are allowed and
    /// convert_js will not check whether they are converted as the same value.
    ///
    /// ```typescript
    /// | string
    /// | [number, boolean]
    /// | { field1?: number, field2: number | undefined, field3: number | null }
    /// | undefined
    /// ```
    Untagged,

    /// `#[convert_js(union)]`
    ///
    /// Similar to `#[convert_js(untagged)]`, except that
    /// `VariantUnit` will be converted as a string literal of the variant name.
    /// Thus, multiple `VariantUnit` are allowed by default.
    ///
    /// This is analogous to [`Union Types` in TypeScript](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types).
    ///
    /// ```typescript
    /// | string
    /// | [number, boolean]
    /// | { field1?: number, field2: number | undefined, field3: number | null }
    /// | "VariantUnit"
    /// ```
    Union,
}

pub struct EnumConvertJsStyleInput {
    pub all_external: Flag,
    pub tag: Option<String>,
    pub content: Option<String>,
    pub untagged: Flag,
    pub union: Flag,
}

impl TryFrom<EnumConvertJsStyleInput> for EnumConvertJsStyle {
    type Error = String;

    fn try_from(value: EnumConvertJsStyleInput) -> Result<Self, Self::Error> {
        let EnumConvertJsStyleInput {
            all_external,
            tag,
            content,
            union,
            untagged,
        } = value;

        if all_external.is_some() {
            crate::util::not_present!(tag, all_external)?;
            crate::util::not_present!(content, all_external)?;
            crate::util::not_present!(union, all_external)?;
            crate::util::not_present!(untagged, all_external)?;

            Ok(EnumConvertJsStyle::AllExternal)
        } else if let Some(tag) = tag {
            crate::util::not_present!(union, tag)?;
            crate::util::not_present!(untagged, tag)?;

            if let Some(content) = content {
                Ok(EnumConvertJsStyle::Adjacent { tag, content })
            } else {
                Ok(EnumConvertJsStyle::Internal { tag })
            }
        } else if union.is_some() {
            crate::util::not_present!(untagged, union)?;
            Ok(EnumConvertJsStyle::Union)
        } else if untagged.is_some() {
            Ok(EnumConvertJsStyle::Untagged)
        } else {
            Ok(EnumConvertJsStyle::External)
        }
    }
}
