use crate::scheme::tinted8::structure::Palette;
use crate::{Color, ColorName, ColorType, ColorVariant, TintedBuilderError};
use serde::Serialize;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
include!(concat!(env!("OUT_DIR"), "/syntax_generated.rs"));

impl Syntax {
    pub fn try_from_basic(basic: &BasicSyntax, palette: &Palette) -> Result<Self, SyntaxError> {
        generated_try_from_basic(basic, palette)
    }

    pub const fn get_property_list() -> &'static [SyntaxKey] {
        SyntaxKey::variants()
    }

    #[allow(clippy::too_many_lines)]
    pub const fn get_color(&self, key: &SyntaxKey) -> &Color {
        match key {
            SyntaxKey::Comment => &self.comment.default,
            SyntaxKey::CommentLine => &self.comment.line,
            SyntaxKey::CommentBlock => &self.comment.block,
            SyntaxKey::CommentDocumentation => &self.comment.documentation,
            SyntaxKey::Invalid => &self.invalid.default,
            SyntaxKey::InvalidDeprecated => &self.invalid.deprecated,
            SyntaxKey::InvalidIllegal => &self.invalid.illegal,
            SyntaxKey::String => &self.string.default,
            SyntaxKey::StringQuoted => &self.string.quoted.default,
            SyntaxKey::StringQuotedSingle => &self.string.quoted.single,
            SyntaxKey::StringQuotedDouble => &self.string.quoted.double,
            SyntaxKey::StringRegexp => &self.string.regexp,
            SyntaxKey::StringTemplate => &self.string.template,
            SyntaxKey::StringInterpolated => &self.string.interpolated,
            SyntaxKey::StringUnquoted => &self.string.unquoted,
            SyntaxKey::StringOther => &self.string.other,
            SyntaxKey::Constant => &self.constant.default,
            SyntaxKey::ConstantNumeric => &self.constant.numeric.default,
            SyntaxKey::ConstantNumericInteger => &self.constant.numeric.integer,
            SyntaxKey::ConstantNumericFloat => &self.constant.numeric.float,
            SyntaxKey::ConstantNumericHex => &self.constant.numeric.hex,
            SyntaxKey::ConstantLanguage => &self.constant.language,
            SyntaxKey::ConstantOther => &self.constant.other,
            SyntaxKey::ConstantCharacter => &self.constant.character.default,
            SyntaxKey::ConstantCharacterEscape => &self.constant.character.escape,
            SyntaxKey::ConstantCharacterEntity => &self.constant.character.entity,
            SyntaxKey::Entity => &self.entity.default,
            SyntaxKey::EntityName => &self.entity.name.default,
            SyntaxKey::EntityNameClass => &self.entity.name.class,
            SyntaxKey::EntityNameFunction => &self.entity.name.function.default,
            SyntaxKey::EntityNameFunctionConstructor => &self.entity.name.function.constructor,
            SyntaxKey::EntityNameLabel => &self.entity.name.label,
            SyntaxKey::EntityNameTag => &self.entity.name.tag,
            SyntaxKey::EntityNameType => &self.entity.name.r#type.default,
            SyntaxKey::EntityNameTypeClass => &self.entity.name.r#type.r#class,
            SyntaxKey::EntityNameTypeEnum => &self.entity.name.r#type.r#enum,
            SyntaxKey::EntityNameTypeStruct => &self.entity.name.r#type.r#struct,
            SyntaxKey::EntityNameNamespace => &self.entity.name.namespace,
            SyntaxKey::EntityNameSection => &self.entity.name.section,
            SyntaxKey::EntityOther => &self.entity.other.default,
            SyntaxKey::EntityOtherAttributeName => &self.entity.other.attribute_name,
            SyntaxKey::EntityOtherInheritedClass => &self.entity.other.inherited_class,
            SyntaxKey::Keyword => &self.keyword.default,
            SyntaxKey::KeywordControl => &self.keyword.control.default,
            SyntaxKey::KeywordControlImport => &self.keyword.control.import,
            SyntaxKey::KeywordControlFlow => &self.keyword.control.flow,
            SyntaxKey::KeywordDeclaration => &self.keyword.declaration,
            SyntaxKey::KeywordOperator => &self.keyword.operator,
            SyntaxKey::KeywordOther => &self.keyword.other,
            SyntaxKey::Storage => &self.storage.default,
            SyntaxKey::StorageType => &self.storage.r#type,
            SyntaxKey::StorageModifier => &self.storage.modifier,
            SyntaxKey::Support => &self.support.default,
            SyntaxKey::SupportFunction => &self.support.function.default,
            SyntaxKey::SupportFunctionBuiltin => &self.support.function.builtin,
            SyntaxKey::SupportClass => &self.support.class,
            SyntaxKey::SupportType => &self.support.r#type,
            SyntaxKey::SupportConstant => &self.support.constant,
            SyntaxKey::SupportVariable => &self.support.variable,
            SyntaxKey::SupportOther => &self.support.other,
            SyntaxKey::Variable => &self.variable.default,
            SyntaxKey::VariableParameter => &self.variable.parameter,
            SyntaxKey::VariableLanguage => &self.variable.language,
            SyntaxKey::VariableOther => &self.variable.other.default,
            SyntaxKey::VariableOtherConstant => &self.variable.other.constant,
            SyntaxKey::VariableOtherObject => &self.variable.other.object,
            SyntaxKey::VariableOtherProperty => &self.variable.other.property,
            SyntaxKey::Punctuation => &self.punctuation.default,
            SyntaxKey::PunctuationSeparator => &self.punctuation.separator,
            SyntaxKey::PunctuationDefinition => &self.punctuation.definition.default,
            SyntaxKey::PunctuationDefinitionString => &self.punctuation.definition.string,
            SyntaxKey::PunctuationDefinitionComment => &self.punctuation.definition.comment,
            SyntaxKey::PunctuationSection => &self.punctuation.section,
            SyntaxKey::Markup => &self.markup.default,
            SyntaxKey::MarkupBold => &self.markup.bold,
            SyntaxKey::MarkupItalic => &self.markup.italic,
            SyntaxKey::MarkupQuote => &self.markup.quote,
            SyntaxKey::MarkupUnderline => &self.markup.underline,
            SyntaxKey::MarkupHeading => &self.markup.heading,
            SyntaxKey::MarkupList => &self.markup.list.default,
            SyntaxKey::MarkupListNumbered => &self.markup.list.numbered,
            SyntaxKey::MarkupListUnnumbered => &self.markup.list.unnumbered,
            SyntaxKey::MarkupLink => &self.markup.link,
            SyntaxKey::MarkupRaw => &self.markup.raw,
            SyntaxKey::MarkupInserted => &self.markup.inserted,
            SyntaxKey::MarkupChanged => &self.markup.changed,
            SyntaxKey::MarkupDeleted => &self.markup.deleted,
            SyntaxKey::Source => &self.source,
            SyntaxKey::Text => &self.text,
            SyntaxKey::Meta => &self.meta.default,
            SyntaxKey::MetaAnnotation => &self.meta.annotation,
            SyntaxKey::MetaFunction => &self.meta.function,
            SyntaxKey::MetaClass => &self.meta.class,
            SyntaxKey::MetaBlock => &self.meta.block,
            SyntaxKey::MetaTag => &self.meta.tag,
            SyntaxKey::MetaType => &self.meta.r#type,
            SyntaxKey::MetaImport => &self.meta.import,
            SyntaxKey::MetaPreprocessor => &self.meta.preprocessor,
            SyntaxKey::MetaEmbedded => &self.meta.embedded,
            SyntaxKey::MetaObject => &self.meta.object,
        }
    }
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for key in SyntaxKey::variants() {
            writeln!(f, "  {key}: #{}", &self.get_color(key).to_hex())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Syntax {
    pub comment: SyntaxComment,
    pub invalid: SyntaxInvalid,
    pub string: SyntaxString,
    pub constant: SyntaxConstant,
    pub entity: SyntaxEntity,
    pub keyword: SyntaxKeyword,
    pub storage: SyntaxStorage,
    pub support: SyntaxSupport,
    pub variable: SyntaxVariable,
    pub punctuation: SyntaxPunctuation,
    pub markup: SyntaxMarkup,
    pub source: Color,
    pub text: Color,
    pub meta: SyntaxMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxStringQuoted {
    pub default: Color,
    pub single: Color,
    pub double: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxComment {
    pub default: Color,
    pub line: Color,
    pub block: Color,
    pub documentation: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxString {
    pub default: Color,
    pub quoted: SyntaxStringQuoted,
    pub regexp: Color,
    pub template: Color,
    pub interpolated: Color,
    pub unquoted: Color,
    pub other: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxConstant {
    pub default: Color,
    pub numeric: SyntaxConstantNumeric,
    pub language: Color,
    pub character: SyntaxConstantCharacter,
    pub other: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxConstantCharacter {
    pub default: Color,
    pub escape: Color,
    pub entity: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxConstantNumeric {
    pub default: Color,
    pub integer: Color,
    pub float: Color,
    pub hex: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntity {
    pub default: Color,
    pub name: SyntaxEntityName,
    pub other: SyntaxEntityOther,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxVariable {
    pub default: Color,
    pub parameter: Color,
    pub language: Color,
    pub other: SyntaxVariableOther,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxVariableOther {
    pub default: Color,
    pub constant: Color,
    pub object: Color,
    pub property: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntityName {
    pub default: Color,
    pub class: Color,
    pub function: SyntaxEntityNameFunction,
    pub label: Color,
    pub tag: Color,
    #[serde(rename = "type")]
    pub r#type: SyntaxEntityNameType,
    pub namespace: Color,
    pub section: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntityNameType {
    pub default: Color,
    pub class: Color,
    pub r#enum: Color,
    pub r#struct: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntityNameFunction {
    pub default: Color,
    pub constructor: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntityOther {
    pub default: Color,
    #[serde(rename = "attribute-name")]
    pub attribute_name: Color,
    #[serde(rename = "inherited-class")]
    pub inherited_class: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxKeyword {
    pub default: Color,
    pub control: SyntaxKeywordControl,
    pub declaration: Color,
    pub operator: Color,
    pub other: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxKeywordControl {
    pub default: Color,
    pub import: Color,
    pub flow: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxStorage {
    pub default: Color,
    #[serde(rename = "type")]
    pub r#type: Color,
    pub modifier: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxSupport {
    pub default: Color,
    pub function: SyntaxSupportFunction,
    pub class: Color,
    #[serde(rename = "type")]
    pub r#type: Color,
    pub constant: Color,
    pub variable: Color,
    pub other: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxSupportFunction {
    pub default: Color,
    pub builtin: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxPunctuation {
    pub default: Color,
    pub separator: Color,
    pub definition: SyntaxPunctuationDefinition,
    pub section: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxPunctuationDefinition {
    pub default: Color,
    pub string: Color,
    pub comment: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxInvalid {
    pub default: Color,
    pub deprecated: Color,
    pub illegal: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxMarkup {
    pub default: Color,
    pub bold: Color,
    pub italic: Color,
    pub quote: Color,
    pub underline: Color,
    pub heading: Color,
    pub list: SyntaxMarkupList,
    pub link: Color,
    pub raw: Color,
    pub inserted: Color,
    pub changed: Color,
    pub deleted: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxMarkupList {
    pub default: Color,
    pub numbered: Color,
    pub unnumbered: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxMeta {
    pub default: Color,
    pub annotation: Color,
    pub function: Color,
    pub class: Color,
    pub block: Color,
    pub tag: Color,
    pub r#type: Color,
    pub import: Color,
    pub preprocessor: Color,
    pub embedded: Color,
    pub object: Color,
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("unable to convert from type: {0}")]
    UnableToConvertFrom(String),
}

impl From<TintedBuilderError> for SyntaxError {
    fn from(error: TintedBuilderError) -> Self {
        Self::UnableToConvertFrom(error.to_string())
    }
}

/// Parse a color with parent inheritance semantics.
///
/// Resolution order:
/// 1. Use and parse `value` if provided.
/// 2. Otherwise, use `parent` if provided (parsed via `parse_or_inherit`).
/// 3. Otherwise, fall back to `default`.
///
/// This supports cases like `string.quoted` inheriting from `string` when the
/// child value is omitted.
///
/// Errors
/// Returns `SyntaxError::UnableToConvertFrom` if a provided string cannot be
/// parsed into a `Color`.
fn parse_or_inherit(value_list: &[Option<&str>], default: &Color) -> Result<Color, SyntaxError> {
    let value_list: Vec<String> = value_list
        .iter()
        .filter_map(|s| s.map(std::string::ToString::to_string))
        .collect();

    value_list.first().map_or_else(
        || Ok(default.clone()),
        |val| {
            Color::new(val, None, None).map_err(|e| SyntaxError::UnableToConvertFrom(e.to_string()))
        },
    )
}
