use crate::scheme::tinted8::structure::Palette;
use crate::scheme::tinted8::yaml::BasicSyntax;
use crate::Color;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum SyntaxKey {
    Comment,
    CommentLine,
    CommentBlock,
    Invalid,
    InvalidDeprecated,
    InvalidIllegal,
    String,
    StringQuoted,
    StringQuotedSingle,
    StringQuotedDouble,
    StringRegexp,
    StringTemplate,
    StringInterpolated,
    StringUnquoted,
    Constant,
    ConstantNumeric,
    ConstantNumericInteger,
    ConstantNumericFloat,
    ConstantNumericHex,
    ConstantNumericExponential,
    ConstantLanguage,
    ConstantLanguageBoolean,
    ConstantOther,
    ConstantCharacter,
    ConstantCharacterEscape,
    ConstantCharacterEntity,
    Entity,
    EntityName,
    EntityNameClass,
    EntityNameFilename,
    EntityNameFunction,
    EntityNameTag,
    EntityNameVariable,
    EntityNameType,
    EntityNameNamespace,
    EntityNameSection,
    EntityOther,
    EntityOtherAttributeName,
    EntityOtherInheritedClass,
    Keyword,
    KeywordControl,
    KeywordDeclaration,
    KeywordOperator,
    KeywordOther,
    Storage,
    StorageType,
    StorageModifier,
    Support,
    SupportFunction,
    SupportClass,
    SupportType,
    SupportConstant,
    SupportVariable,
    Variable,
    VariableParameter,
    VariableLanguage,
    VariableFunction,
    Punctuation,
    PunctuationAccessor,
    PunctuationSeparator,
    PunctuationTerminator,
    Markup,
    MarkupHeading,
    MarkupBold,
    MarkupCode,
    MarkupItalic,
    MarkupQuote,
    MarkupUnderline,
    MarkupList,
    MarkupLink,
    MarkupRaw,
    Diff,
    DiffAdded,
    DiffChanged,
    DiffDeleted,
}

impl SyntaxKey {
    const fn as_str(&self) -> &str {
        match self {
            Self::Comment => "comment",
            Self::CommentLine => "comment.block",
            Self::CommentBlock => "comment.line",
            Self::Invalid => "invalid",
            Self::InvalidDeprecated => "invalid.deprecated",
            Self::InvalidIllegal => "invalid.illegal",
            Self::String => "string",
            Self::StringQuoted => "string.quoted",
            Self::StringQuotedSingle => "string.quoted.single",
            Self::StringQuotedDouble => "string.quoted.double",
            Self::StringRegexp => "string.regexp",
            Self::StringTemplate => "string.template",
            Self::StringInterpolated => "string.interpolated",
            Self::StringUnquoted => "string.unquoted",
            Self::Constant => "constant",
            Self::ConstantNumeric => "constant.numeric",
            Self::ConstantNumericInteger => "constant.numeric.integer",
            Self::ConstantNumericFloat => "constant.numeric.float",
            Self::ConstantNumericHex => "constant.numeric.hex",
            Self::ConstantNumericExponential => "constant.numeric.exponential",
            Self::ConstantLanguage => "constant.language",
            Self::ConstantLanguageBoolean => "constant.language.boolean",
            Self::ConstantOther => "constant.other",
            Self::ConstantCharacter => "constant.character",
            Self::ConstantCharacterEscape => "constant.character.escape",
            Self::ConstantCharacterEntity => "constant.character.entity",
            Self::Entity => "entity",
            Self::EntityName => "entity.name",
            Self::EntityNameClass => "entity.name.class",
            Self::EntityNameFilename => "entity.name.filename",
            Self::EntityNameFunction => "entity.name.function",
            Self::EntityNameTag => "entity.name.tag",
            Self::EntityNameVariable => "entity.name.variable",
            Self::EntityNameType => "entity.name.type",
            Self::EntityNameNamespace => "entity.name.namespace",
            Self::EntityNameSection => "entity.name.section",
            Self::EntityOther => "entity.other",
            Self::EntityOtherAttributeName => "entity.other.attribute-name",
            Self::EntityOtherInheritedClass => "entity.other.inherited-class",
            Self::Keyword => "keyword",
            Self::KeywordControl => "keyword.control",
            Self::KeywordDeclaration => "keyword.declaration",
            Self::KeywordOperator => "keyword.operator",
            Self::KeywordOther => "keyword.other",
            Self::Storage => "storage",
            Self::StorageType => "storage.type",
            Self::StorageModifier => "storage.modifier",
            Self::Support => "support",
            Self::SupportFunction => "support.function",
            Self::SupportClass => "support.class",
            Self::SupportType => "support.type",
            Self::SupportConstant => "support.constant",
            Self::SupportVariable => "support.variable",
            Self::Variable => "variable",
            Self::VariableParameter => "variable.parameter",
            Self::VariableLanguage => "variable.language",
            Self::VariableFunction => "variable.function",
            Self::Punctuation => "punctuation",
            Self::PunctuationAccessor => "punctuation.accessor",
            Self::PunctuationSeparator => "punctuation.separator",
            Self::PunctuationTerminator => "punctuation.terminator",
            Self::Markup => "markup",
            Self::MarkupHeading => "markup.heading",
            Self::MarkupBold => "markup.bold",
            Self::MarkupCode => "markup.code",
            Self::MarkupItalic => "markup.italic",
            Self::MarkupQuote => "markup.quote",
            Self::MarkupUnderline => "markup.underline",
            Self::MarkupList => "markup.list",
            Self::MarkupLink => "markup.link",
            Self::MarkupRaw => "markup.raw",
            Self::Diff => "diff",
            Self::DiffAdded => "diff.added",
            Self::DiffChanged => "diff.changed",
            Self::DiffDeleted => "diff.deleted",
        }
    }

    pub const fn variants() -> &'static [Self] {
        &[
            Self::Comment,
            Self::CommentLine,
            Self::CommentBlock,
            Self::Invalid,
            Self::InvalidDeprecated,
            Self::InvalidIllegal,
            Self::String,
            Self::StringQuoted,
            Self::StringQuotedSingle,
            Self::StringQuotedDouble,
            Self::StringRegexp,
            Self::StringTemplate,
            Self::StringInterpolated,
            Self::StringUnquoted,
            Self::Constant,
            Self::ConstantNumeric,
            Self::ConstantNumericInteger,
            Self::ConstantNumericFloat,
            Self::ConstantNumericHex,
            Self::ConstantNumericExponential,
            Self::ConstantLanguage,
            Self::ConstantLanguageBoolean,
            Self::ConstantOther,
            Self::ConstantCharacter,
            Self::ConstantCharacterEscape,
            Self::ConstantCharacterEntity,
            Self::Entity,
            Self::EntityName,
            Self::EntityNameClass,
            Self::EntityNameFunction,
            Self::EntityNameTag,
            Self::EntityNameVariable,
            Self::EntityNameType,
            Self::EntityNameNamespace,
            Self::EntityNameSection,
            Self::EntityOther,
            Self::EntityOtherAttributeName,
            Self::EntityOtherInheritedClass,
            Self::Keyword,
            Self::KeywordControl,
            Self::KeywordDeclaration,
            Self::KeywordOperator,
            Self::KeywordOther,
            Self::Storage,
            Self::StorageType,
            Self::StorageModifier,
            Self::Support,
            Self::SupportFunction,
            Self::SupportClass,
            Self::SupportType,
            Self::SupportConstant,
            Self::SupportVariable,
            Self::Variable,
            Self::VariableParameter,
            Self::VariableLanguage,
            Self::VariableFunction,
            Self::Punctuation,
            Self::PunctuationAccessor,
            Self::PunctuationSeparator,
            Self::PunctuationTerminator,
            Self::Markup,
            Self::MarkupHeading,
            Self::MarkupBold,
            Self::MarkupCode,
            Self::MarkupItalic,
            Self::MarkupQuote,
            Self::MarkupUnderline,
            Self::MarkupList,
            Self::MarkupLink,
            Self::MarkupRaw,
            Self::Diff,
            Self::DiffAdded,
            Self::DiffChanged,
            Self::DiffDeleted,
        ]
    }
}

impl fmt::Display for SyntaxKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())?;

        Ok(())
    }
}

#[allow(clippy::too_many_lines)]
impl Syntax {
    /// Create a new Syntax struct with default colors
    ///
    /// These default colors are set according to tinted8 0.1.0 styling spec
    pub fn new(palette: &Palette) -> Self {
        let string_normal = palette.yellow_normal.clone();
        let constant_normal = palette.magenta_normal.clone();
        let entity_normal = palette.white_normal.clone();
        let keyword_normal = palette.magenta_normal.clone();
        let storage_normal = palette.blue_normal.clone();
        let markup_normal = palette.cyan_normal.clone();
        let variable_normal = palette.white_normal.clone();
        let comment_normal = palette.gray_normal.clone();
        let punctuation_normal = palette.gray_dim.clone();
        let syntax_support_normal = palette.blue_normal.clone();
        let syntax_comment = SyntaxComment {
            line: comment_normal.clone(),
            block: comment_normal.clone(),
            default: comment_normal,
        };
        let syntax_string = SyntaxString {
            quoted: SyntaxStringQuoted {
                default: string_normal.clone(),
                single: string_normal.clone(),
                double: string_normal.clone(),
            },
            regexp: palette.cyan_normal.clone(),
            template: string_normal.clone(),
            interpolated: string_normal.clone(),
            unquoted: string_normal.clone(),
            default: string_normal,
        };
        let syntax_constant = SyntaxConstant {
            numeric: SyntaxConstantNumeric {
                integer: constant_normal.clone(),
                float: constant_normal.clone(),
                hex: constant_normal.clone(),
                exponential: constant_normal.clone(),
                default: constant_normal.clone(),
            },
            language: SyntaxConstantLanguage {
                default: constant_normal.clone(),
                boolean: constant_normal.clone(),
            },
            character: SyntaxConstantCharacter {
                default: constant_normal.clone(),
                escape: constant_normal.clone(),
                entity: constant_normal.clone(),
            },
            other: constant_normal.clone(),
            default: constant_normal,
        };
        let syntax_entity = SyntaxEntity {
            name: SyntaxEntityName {
                default: entity_normal.clone(),
                class: entity_normal.clone(),
                filename: entity_normal.clone(),
                function: palette.blue_normal.clone(),
                tag: entity_normal.clone(),
                variable: entity_normal.clone(),
                r#type: entity_normal.clone(),
                namespace: entity_normal.clone(),
                section: palette.cyan_normal.clone(),
            },
            other: SyntaxEntityOther {
                default: entity_normal.clone(),
                attribute_name: palette.magenta_normal.clone(),
                inherited_class: entity_normal.clone(),
            },
            default: entity_normal,
        };
        let syntax_keyword = SyntaxKeyword {
            control: keyword_normal.clone(),
            declaration: keyword_normal.clone(),
            operator: keyword_normal.clone(),
            other: keyword_normal.clone(),
            default: keyword_normal,
        };
        let syntax_storage = SyntaxStorage {
            r#type: storage_normal.clone(),
            modifier: storage_normal.clone(),
            default: storage_normal,
        };
        let syntax_support = SyntaxSupport {
            function: syntax_support_normal.clone(),
            class: syntax_support_normal.clone(),
            r#type: syntax_support_normal.clone(),
            constant: palette.magenta_normal.clone(),
            variable: syntax_support_normal.clone(),
            default: syntax_support_normal,
        };
        let syntax_variable = SyntaxVariable {
            parameter: palette.cyan_bright.clone(),
            language: palette.magenta_normal.clone(),
            function: palette.cyan_normal.clone(),
            default: variable_normal,
        };
        let syntax_punctuation = SyntaxPunctuation {
            accessor: palette.gray_bright.clone(),
            separator: punctuation_normal.clone(),
            terminator: punctuation_normal.clone(),
            default: punctuation_normal,
        };
        let syntax_invalid = SyntaxInvalid {
            default: palette.red_bright.clone(),
            deprecated: palette.yellow_bright.clone(),
            illegal: palette.red_bright.clone(),
        };
        let syntax_markup = SyntaxMarkup {
            heading: markup_normal.clone(),
            bold: markup_normal.clone(),
            code: markup_normal.clone(),
            italic: markup_normal.clone(),
            quote: markup_normal.clone(),
            underline: markup_normal.clone(),
            list: markup_normal.clone(),
            link: markup_normal.clone(),
            raw: markup_normal.clone(),
            default: markup_normal,
        };
        let syntax_diff = SyntaxDiff {
            default: palette.cyan_normal.clone(),
            added: palette.green_bright.clone(),
            changed: palette.magenta_bright.clone(),
            deleted: palette.red_bright.clone(),
        };

        Self {
            comment: syntax_comment,
            invalid: syntax_invalid,
            string: syntax_string,
            constant: syntax_constant,
            entity: syntax_entity,
            keyword: syntax_keyword,
            storage: syntax_storage,
            support: syntax_support,
            variable: syntax_variable,
            punctuation: syntax_punctuation,
            markup: syntax_markup,
            diff: syntax_diff,
        }
    }
    /// Tries to convert `BasicSyntax` (create after deserializing yaml) to Syntax
    pub fn try_from_basic(basic: &BasicSyntax, palette: &Palette) -> Result<Self, SyntaxError> {
        let default_syntax = Self::new(palette);
        let comment = SyntaxComment {
            default: parse_or_default(basic.comment.as_deref(), &default_syntax.comment.default)?,
            line: parse_or_inherit(
                basic.comment_line.as_deref(),
                basic.comment.as_deref(),
                &default_syntax.comment.default,
            )?,
            block: parse_or_inherit(
                basic.string_template.as_deref(),
                basic.string.as_deref(),
                &default_syntax.string.template,
            )?,
        };
        let invalid = SyntaxInvalid {
            default: parse_or_default(basic.invalid.as_deref(), &default_syntax.invalid.default)?,
            deprecated: parse_or_inherit(
                basic.invalid_deprecated.as_deref(),
                basic.invalid.as_deref(),
                &default_syntax.invalid.deprecated,
            )?,
            illegal: parse_or_inherit(
                basic.invalid_illegal.as_deref(),
                basic.invalid.as_deref(),
                &default_syntax.invalid.illegal,
            )?,
        };
        let string = SyntaxString {
            default: parse_or_default(basic.string.as_deref(), &default_syntax.string.default)?,
            quoted: SyntaxStringQuoted {
                default: parse_or_inherit(
                    basic.string_quoted.as_deref(),
                    basic.string.as_deref(),
                    &default_syntax.string.quoted.default,
                )?,
                single: parse_or_inherit(
                    basic.string_quoted_single.as_deref(),
                    basic.string.as_deref(),
                    &default_syntax.string.quoted.single,
                )?,
                double: parse_or_inherit(
                    basic.string_quoted_double.as_deref(),
                    basic.string.as_deref(),
                    &default_syntax.string.quoted.double,
                )?,
            },
            regexp: parse_or_inherit(
                basic.string_regexp.as_deref(),
                basic.string.as_deref(),
                &default_syntax.string.regexp,
            )?,
            template: parse_or_inherit(
                basic.string_template.as_deref(),
                basic.string.as_deref(),
                &default_syntax.string.template,
            )?,
            interpolated: parse_or_inherit(
                basic.string_interpolated.as_deref(),
                basic.string.as_deref(),
                &default_syntax.string.interpolated,
            )?,
            unquoted: parse_or_inherit(
                basic.string_unquoted.as_deref(),
                basic.string.as_deref(),
                &default_syntax.string.unquoted,
            )?,
        };
        let constant = SyntaxConstant {
            default: parse_or_default(basic.constant.as_deref(), &default_syntax.constant.default)?,
            numeric: SyntaxConstantNumeric {
                default: parse_or_inherit(
                    basic.constant_numeric.as_deref(),
                    basic.constant.as_deref(),
                    &default_syntax.constant.numeric.default,
                )?,
                integer: parse_or_inherit(
                    basic.constant_numeric_integer.as_deref(),
                    basic.constant_numeric.as_deref(),
                    &default_syntax.constant.numeric.integer,
                )?,
                float: parse_or_inherit(
                    basic.constant_numeric_float.as_deref(),
                    basic.constant_numeric.as_deref(),
                    &default_syntax.constant.numeric.float,
                )?,
                hex: parse_or_inherit(
                    basic.constant_numeric_hex.as_deref(),
                    basic.constant_numeric.as_deref(),
                    &default_syntax.constant.numeric.hex,
                )?,
                exponential: parse_or_inherit(
                    basic.constant_numeric_exponential.as_deref(),
                    basic.constant_numeric.as_deref(),
                    &default_syntax.constant.numeric.exponential,
                )?,
            },
            language: SyntaxConstantLanguage {
                default: parse_or_inherit(
                    basic.constant_language.as_deref(),
                    basic.constant.as_deref(),
                    &default_syntax.constant.language.default,
                )?,
                boolean: parse_or_inherit(
                    basic.constant_language_boolean.as_deref(),
                    basic.constant_language.as_deref(),
                    &default_syntax.constant.language.boolean,
                )?,
            },
            character: SyntaxConstantCharacter {
                default: parse_or_inherit(
                    basic.constant_character.as_deref(),
                    basic.constant.as_deref(),
                    &default_syntax.constant.character.default,
                )?,
                escape: parse_or_inherit(
                    basic.constant_character_escape.as_deref(),
                    basic.constant_character.as_deref(),
                    &default_syntax.constant.character.escape,
                )?,
                entity: parse_or_inherit(
                    basic.constant_character_entity.as_deref(),
                    basic.constant_character.as_deref(),
                    &default_syntax.constant.character.entity,
                )?,
            },
            other: parse_or_inherit(
                basic.constant_other.as_deref(),
                basic.constant.as_deref(),
                &default_syntax.constant.other,
            )?,
        };
        let entity = SyntaxEntity {
            default: parse_or_default(basic.entity.as_deref(), &default_syntax.entity.default)?,
            name: SyntaxEntityName {
                default: parse_or_inherit(
                    basic.entity_name.as_deref(),
                    basic.entity.as_deref(),
                    &default_syntax.entity.name.default,
                )?,
                class: parse_or_inherit(
                    basic.entity_name_class.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.default,
                )?,
                filename: parse_or_inherit(
                    basic.entity_name_filename.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.filename,
                )?,
                function: parse_or_inherit(
                    basic.entity_name_function.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.function,
                )?,
                tag: parse_or_inherit(
                    basic.entity_name_tag.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.tag,
                )?,
                variable: parse_or_inherit(
                    basic.entity_name_variable.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.variable,
                )?,
                r#type: parse_or_inherit(
                    basic.entity_name_type.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.r#type,
                )?,
                namespace: parse_or_inherit(
                    basic.entity_name_namespace.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.namespace,
                )?,
                section: parse_or_inherit(
                    basic.entity_name_section.as_deref(),
                    basic.entity_name.as_deref(),
                    &default_syntax.entity.name.section,
                )?,
            },
            other: SyntaxEntityOther {
                default: parse_or_inherit(
                    basic.entity_other.as_deref(),
                    basic.entity.as_deref(),
                    &default_syntax.entity.other.default,
                )?,
                attribute_name: parse_or_default(
                    basic.entity_other_attribute_name.as_deref(),
                    &default_syntax.entity.other.attribute_name,
                )?,
                inherited_class: parse_or_inherit(
                    basic.entity_other_inherited_class.as_deref(),
                    basic.entity_other.as_deref(),
                    &default_syntax.entity.other.inherited_class,
                )?,
            },
        };
        let keyword = SyntaxKeyword {
            default: parse_or_default(basic.keyword.as_deref(), &default_syntax.keyword.default)?,
            control: parse_or_inherit(
                basic.keyword_control.as_deref(),
                basic.keyword.as_deref(),
                &default_syntax.keyword.control,
            )?,
            declaration: parse_or_inherit(
                basic.keyword_declaration.as_deref(),
                basic.keyword.as_deref(),
                &default_syntax.keyword.declaration,
            )?,
            operator: parse_or_inherit(
                basic.keyword_operator.as_deref(),
                basic.keyword.as_deref(),
                &default_syntax.keyword.operator,
            )?,
            other: parse_or_inherit(
                basic.keyword_other.as_deref(),
                basic.keyword.as_deref(),
                &default_syntax.keyword.other,
            )?,
        };
        let storage = SyntaxStorage {
            default: parse_or_default(basic.storage.as_deref(), &default_syntax.storage.default)?,
            r#type: parse_or_inherit(
                basic.storage_type.as_deref(),
                basic.storage.as_deref(),
                &default_syntax.storage.r#type,
            )?,
            modifier: parse_or_inherit(
                basic.storage_modifier.as_deref(),
                basic.storage.as_deref(),
                &default_syntax.storage.modifier,
            )?,
        };
        let support = SyntaxSupport {
            default: parse_or_default(basic.support.as_deref(), &default_syntax.support.default)?,
            function: parse_or_inherit(
                basic.support_function.as_deref(),
                basic.support.as_deref(),
                &default_syntax.support.function,
            )?,
            class: parse_or_inherit(
                basic.support_class.as_deref(),
                basic.support.as_deref(),
                &default_syntax.support.class,
            )?,
            r#type: parse_or_inherit(
                basic.support_type.as_deref(),
                basic.support.as_deref(),
                &default_syntax.support.r#type,
            )?,
            constant: parse_or_inherit(
                basic.support_constant.as_deref(),
                basic.support.as_deref(),
                &default_syntax.support.constant,
            )?,
            variable: parse_or_inherit(
                basic.support_variable.as_deref(),
                basic.support.as_deref(),
                &default_syntax.support.variable,
            )?,
        };
        let variable = SyntaxVariable {
            default: parse_or_default(basic.variable.as_deref(), &default_syntax.variable.default)?,
            parameter: parse_or_inherit(
                basic.variable_parameter.as_deref(),
                basic.variable.as_deref(),
                &default_syntax.variable.parameter,
            )?,
            language: parse_or_inherit(
                basic.variable_language.as_deref(),
                basic.variable.as_deref(),
                &default_syntax.variable.language,
            )?,
            function: parse_or_inherit(
                basic.variable_function.as_deref(),
                basic.variable.as_deref(),
                &default_syntax.variable.function,
            )?,
        };
        let punctuation = SyntaxPunctuation {
            default: parse_or_default(
                basic.punctuation.as_deref(),
                &default_syntax.punctuation.default,
            )?,
            accessor: parse_or_inherit(
                basic.punctuation_accessor.as_deref(),
                basic.punctuation.as_deref(),
                &default_syntax.punctuation.accessor,
            )?,
            separator: parse_or_inherit(
                basic.punctuation_separator.as_deref(),
                basic.punctuation.as_deref(),
                &default_syntax.punctuation.separator,
            )?,
            terminator: parse_or_inherit(
                basic.punctuation_terminator.as_deref(),
                basic.punctuation.as_deref(),
                &default_syntax.punctuation.terminator,
            )?,
        };
        let markup = SyntaxMarkup {
            default: parse_or_default(basic.markup.as_deref(), &default_syntax.markup.default)?,
            heading: parse_or_inherit(
                basic.markup_heading.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.heading,
            )?,
            bold: parse_or_inherit(
                basic.markup_bold.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.bold,
            )?,
            code: parse_or_inherit(
                basic.markup_code.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.code,
            )?,
            italic: parse_or_inherit(
                basic.markup_italic.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.italic,
            )?,
            quote: parse_or_inherit(
                basic.markup_quote.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.quote,
            )?,
            underline: parse_or_inherit(
                basic.markup_underline.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.underline,
            )?,
            list: parse_or_inherit(
                basic.markup_list.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.list,
            )?,
            link: parse_or_inherit(
                basic.markup_link.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.link,
            )?,
            raw: parse_or_inherit(
                basic.markup_raw.as_deref(),
                basic.markup.as_deref(),
                &default_syntax.markup.raw,
            )?,
        };
        let diff = SyntaxDiff {
            default: parse_or_default(basic.diff.as_deref(), &default_syntax.diff.default)?,
            added: parse_or_default(basic.diff_added.as_deref(), &default_syntax.diff.added)?,
            changed: parse_or_default(basic.diff_changed.as_deref(), &default_syntax.diff.changed)?,
            deleted: parse_or_default(basic.diff_deleted.as_deref(), &default_syntax.diff.deleted)?,
        };

        Ok(Self {
            comment,
            invalid,
            string,
            constant,
            entity,
            keyword,
            storage,
            support,
            variable,
            punctuation,
            markup,
            diff,
        })
    }

    pub const fn get_property_list() -> &'static [SyntaxKey] {
        SyntaxKey::variants()
    }

    // For Display trait
    pub const fn get_color(&self, key: &SyntaxKey) -> &Color {
        match key {
            SyntaxKey::Comment => &self.comment.default,
            SyntaxKey::CommentLine => &self.comment.line,
            SyntaxKey::CommentBlock => &self.comment.block,
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
            SyntaxKey::Constant => &self.constant.default,
            SyntaxKey::ConstantNumeric => &self.constant.numeric.default,
            SyntaxKey::ConstantNumericInteger => &self.constant.numeric.integer,
            SyntaxKey::ConstantNumericFloat => &self.constant.numeric.float,
            SyntaxKey::ConstantNumericHex => &self.constant.numeric.hex,
            SyntaxKey::ConstantNumericExponential => &self.constant.numeric.exponential,
            SyntaxKey::ConstantLanguage => &self.constant.language.default,
            SyntaxKey::ConstantLanguageBoolean => &self.constant.language.boolean,
            SyntaxKey::ConstantOther => &self.constant.other,
            SyntaxKey::ConstantCharacter => &self.constant.character.default,
            SyntaxKey::ConstantCharacterEscape => &self.constant.character.escape,
            SyntaxKey::ConstantCharacterEntity => &self.constant.character.entity,
            SyntaxKey::Entity => &self.entity.default,
            SyntaxKey::EntityName => &self.entity.name.default,
            SyntaxKey::EntityNameClass => &self.entity.name.class,
            SyntaxKey::EntityNameFilename => &self.entity.name.filename,
            SyntaxKey::EntityNameFunction => &self.entity.name.function,
            SyntaxKey::EntityNameTag => &self.entity.name.tag,
            SyntaxKey::EntityNameVariable => &self.entity.name.variable,
            SyntaxKey::EntityNameType => &self.entity.name.r#type,
            SyntaxKey::EntityNameNamespace => &self.entity.name.namespace,
            SyntaxKey::EntityNameSection => &self.entity.name.section,
            SyntaxKey::EntityOther => &self.entity.other.default,
            SyntaxKey::EntityOtherAttributeName => &self.entity.other.attribute_name,
            SyntaxKey::EntityOtherInheritedClass => &self.entity.other.inherited_class,
            SyntaxKey::Keyword => &self.keyword.default,
            SyntaxKey::KeywordControl => &self.keyword.control,
            SyntaxKey::KeywordDeclaration => &self.keyword.declaration,
            SyntaxKey::KeywordOperator => &self.keyword.operator,
            SyntaxKey::KeywordOther => &self.keyword.other,
            SyntaxKey::Storage => &self.storage.default,
            SyntaxKey::StorageType => &self.storage.r#type,
            SyntaxKey::StorageModifier => &self.storage.modifier,
            SyntaxKey::Support => &self.support.default,
            SyntaxKey::SupportFunction => &self.support.function,
            SyntaxKey::SupportClass => &self.support.class,
            SyntaxKey::SupportType => &self.support.r#type,
            SyntaxKey::SupportConstant => &self.support.constant,
            SyntaxKey::SupportVariable => &self.support.variable,
            SyntaxKey::Variable => &self.variable.default,
            SyntaxKey::VariableParameter => &self.variable.parameter,
            SyntaxKey::VariableLanguage => &self.variable.language,
            SyntaxKey::VariableFunction => &self.variable.function,
            SyntaxKey::Punctuation => &self.punctuation.default,
            SyntaxKey::PunctuationAccessor => &self.punctuation.accessor,
            SyntaxKey::PunctuationSeparator => &self.punctuation.separator,
            SyntaxKey::PunctuationTerminator => &self.punctuation.terminator,
            SyntaxKey::Markup => &self.markup.default,
            SyntaxKey::MarkupHeading => &self.markup.heading,
            SyntaxKey::MarkupBold => &self.markup.bold,
            SyntaxKey::MarkupCode => &self.markup.code,
            SyntaxKey::MarkupItalic => &self.markup.italic,
            SyntaxKey::MarkupQuote => &self.markup.quote,
            SyntaxKey::MarkupUnderline => &self.markup.underline,
            SyntaxKey::MarkupList => &self.markup.list,
            SyntaxKey::MarkupLink => &self.markup.link,
            SyntaxKey::MarkupRaw => &self.markup.raw,
            SyntaxKey::Diff => &self.diff.default,
            SyntaxKey::DiffAdded => &self.diff.added,
            SyntaxKey::DiffChanged => &self.diff.changed,
            SyntaxKey::DiffDeleted => &self.diff.deleted,
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
    pub diff: SyntaxDiff,
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
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxString {
    pub default: Color,
    pub quoted: SyntaxStringQuoted,
    pub regexp: Color,
    pub template: Color,
    pub interpolated: Color,
    pub unquoted: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxConstant {
    pub default: Color,
    pub numeric: SyntaxConstantNumeric,
    pub language: SyntaxConstantLanguage,
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
pub struct SyntaxConstantLanguage {
    pub default: Color,
    pub boolean: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxConstantNumeric {
    pub default: Color,
    pub integer: Color,
    pub float: Color,
    pub hex: Color,
    pub exponential: Color,
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
    pub function: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxEntityName {
    pub default: Color,
    pub class: Color,
    pub filename: Color,
    pub function: Color,
    pub tag: Color,
    pub variable: Color,
    #[serde(rename = "type")]
    pub r#type: Color,
    pub namespace: Color,
    pub section: Color,
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
    pub control: Color,
    pub declaration: Color,
    pub operator: Color,
    pub other: Color,
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
    pub function: Color,
    pub class: Color,
    #[serde(rename = "type")]
    pub r#type: Color,
    pub constant: Color,
    pub variable: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxPunctuation {
    pub default: Color,
    pub accessor: Color,
    pub separator: Color,
    pub terminator: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxInvalid {
    pub default: Color,
    pub deprecated: Color,
    pub illegal: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxDiff {
    pub default: Color,
    pub added: Color,
    pub changed: Color,
    pub deleted: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntaxMarkup {
    pub default: Color,
    pub heading: Color,
    pub bold: Color,
    pub code: Color,
    pub italic: Color,
    pub quote: Color,
    pub underline: Color,
    pub list: Color,
    pub link: Color,
    pub raw: Color,
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("unable to convert from type: {0}")]
    UnableToConvertFrom(String),
}

/// Parse a color string or use a provided default.
///
/// - `value`: Optional color string (e.g. "#aabbcc" or "aabbcc"). If `Some`,
///   it is parsed into a `Color`.
/// - `default`: Fallback `Color` used when `value` is `None`.
///
/// Returns the parsed `Color` or clones `default` when `value` is not provided.
///
/// Errors
/// Returns `SyntaxError::UnableToConvertFrom` if the provided `value` cannot be
/// parsed into a `Color`.
fn parse_or_default(value: Option<&str>, default: &Color) -> Result<Color, SyntaxError> {
    value.map_or_else(
        || Ok(default.clone()),
        |s| {
            let color = default.clone();
            Color::new(s, Some(color.name), Some(color.variant))
                .map_err(|e| SyntaxError::UnableToConvertFrom(e.to_string()))
        },
    )
}

/// Parse a color with parent inheritance semantics.
///
/// Resolution order:
/// 1. Use and parse `value` if provided.
/// 2. Otherwise, use `parent` if provided (parsed via `parse_or_default`).
/// 3. Otherwise, fall back to `default`.
///
/// This supports cases like `string.quoted` inheriting from `string` when the
/// child value is omitted.
///
/// Errors
/// Returns `SyntaxError::UnableToConvertFrom` if a provided string cannot be
/// parsed into a `Color`.
fn parse_or_inherit(
    value: Option<&str>,
    parent: Option<&str>,
    default: &Color,
) -> Result<Color, SyntaxError> {
    value.map_or_else(
        || parse_or_default(parent, default),
        |s| {
            let color = default.clone();
            Color::new(s, Some(color.name), Some(color.variant))
                .map_err(|e| SyntaxError::UnableToConvertFrom(e.to_string()))
        },
    )
}
