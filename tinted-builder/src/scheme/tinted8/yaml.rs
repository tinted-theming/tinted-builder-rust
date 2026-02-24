use crate::scheme::tinted8::SchemeSystem;
use crate::scheme::SchemeVariant;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{Mapping, Value};
use std::fmt;

#[derive(Serialize)]
pub struct Tinted8Scheme {
    pub scheme: Meta,
    pub palette: BasicPalette,
    pub variant: SchemeVariant,

    pub syntax: Option<BasicSyntax>,
    pub ui: Option<BasicUi>,
    pub family: Option<String>,
    pub style: Option<String>,
}

// Helper type that mirrors `Tinted8Scheme` for inner deserialization.
#[derive(Deserialize)]
struct Tinted8SchemeHelper {
    pub scheme: Meta,
    pub palette: BasicPalette,
    pub variant: SchemeVariant,

    pub syntax: Option<BasicSyntax>,
    pub ui: Option<BasicUi>,
    pub family: Option<String>,
    pub style: Option<String>,
}

impl From<Tinted8SchemeHelper> for Tinted8Scheme {
    fn from(h: Tinted8SchemeHelper) -> Self {
        Self {
            scheme: h.scheme,
            palette: h.palette,
            variant: h.variant,
            syntax: h.syntax,
            ui: h.ui,
            family: h.family,
            style: h.style,
        }
    }
}

impl<'de> Deserialize<'de> for Tinted8Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut value = Value::deserialize(deserializer)?;

        if let Value::Mapping(root) = &mut value {
            // If there is a `syntax` mapping, flatten nested keys to dotted notation.
            if let Some(syntax_val) = root.get_mut(Value::String("syntax".to_string())) {
                if let Value::Mapping(syntax_map) = syntax_val {
                    let mut flattened = flatten_mapping(None, syntax_map);

                    // Overlay original non-mapping entries to take precedence.
                    for (k, v) in syntax_map.iter() {
                        if !matches!(v, Value::Mapping(_)) {
                            flattened.insert(k.clone(), v.clone());
                        }
                    }

                    *syntax_val = Value::Mapping(flattened);
                }
            }
            // If there is a `ui` mapping, flatten nested keys to dotted notation.
            if let Some(ui_val) = root.get_mut(Value::String("ui".to_string())) {
                if let Value::Mapping(ui_map) = ui_val {
                    let mut flattened = flatten_mapping(None, ui_map);

                    // Overlay original non-mapping entries to take precedence.
                    for (k, v) in ui_map.iter() {
                        if !matches!(v, Value::Mapping(_)) {
                            flattened.insert(k.clone(), v.clone());
                        }
                    }

                    *ui_val = Value::Mapping(flattened);
                }
            }
        }

        let helper: Tinted8SchemeHelper = serde_yaml::from_value(value)
            .map_err(|e| serde::de::Error::custom(format!("unable to deserialize yaml: {e}")))?;

        Ok(helper.into())
    }
}

// Recursively flattens a YAML mapping into dotted keys.
// For example: { entity: { name: "#fff", other.attribute-name: "#eee" } }
// becomes: { "entity.name": "#fff", "entity.other.attribute-name": "#eee" }
fn flatten_mapping(prefix: Option<&str>, map: &Mapping) -> Mapping {
    let mut flattened_map = Mapping::new();

    for (key, value) in map {
        // Only handle string keys; ignore others.
        let key_str = match key {
            Value::String(s) => s.as_str(),
            _ => continue,
        };

        let joined = match prefix {
            Some(p) if !p.is_empty() => format!("{p}.{key_str}"),
            _ => key_str.to_string(),
        };

        match value {
            Value::Mapping(child) => {
                let flattened_inner_map = flatten_mapping(Some(&joined), child);

                flattened_map.extend(flattened_inner_map);
            }
            other => {
                flattened_map.insert(Value::String(joined), other.clone());
            }
        }
    }

    flattened_map
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BasicUi {
    #[serde(rename = "background.normal")]
    pub background_normal: Option<String>,
    #[serde(rename = "background.dark")]
    pub background_dark: Option<String>,
    #[serde(rename = "background.light")]
    pub background_light: Option<String>,
    pub deprecated: Option<String>,
    #[serde(rename = "accent")]
    pub accent: Option<String>,
    #[serde(rename = "border")]
    pub border: Option<String>,
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    #[serde(rename = "foreground.normal")]
    pub foreground_normal: Option<String>,
    #[serde(rename = "foreground.dark")]
    pub foreground_dark: Option<String>,
    #[serde(rename = "foreground.light")]
    pub foreground_light: Option<String>,
    #[serde(rename = "gutter.background")]
    pub gutter_background: Option<String>,
    #[serde(rename = "gutter.foreground")]
    pub gutter_foreground: Option<String>,
    #[serde(rename = "highlight.line.background")]
    pub highlight_line_background: Option<String>,
    #[serde(rename = "highlight.line.foreground")]
    pub highlight_line_foreground: Option<String>,
    #[serde(rename = "highlight.search.background")]
    pub highlight_search_background: Option<String>,
    #[serde(rename = "highlight.search.foreground")]
    pub highlight_search_foreground: Option<String>,
    #[serde(rename = "highlight.text.background")]
    pub highlight_text_background: Option<String>,
    #[serde(rename = "highlight.text.foreground")]
    pub highlight_text_foreground: Option<String>,
    #[serde(rename = "highlight.text.active-background")]
    pub highlight_text_active_background: Option<String>,
    #[serde(rename = "highlight.text.active-foreground")]
    pub highlight_text_active_foreground: Option<String>,
    #[serde(rename = "indent-guide.background")]
    pub indent_guide_background: Option<String>,
    #[serde(rename = "indent-guide.active-background")]
    pub indent_guide_active_background: Option<String>,
    #[serde(rename = "line.background")]
    pub line_background: Option<String>,
    #[serde(rename = "line.foreground")]
    pub line_foreground: Option<String>,
    #[serde(rename = "link")]
    pub link: Option<String>,
    #[serde(rename = "selection.background")]
    pub selection_background: Option<String>,
    #[serde(rename = "selection.foreground")]
    pub selection_foreground: Option<String>,
    #[serde(rename = "selection.inactive-background")]
    pub selection_inactive_background: Option<String>,
    #[serde(rename = "status.error")]
    pub status_error: Option<String>,
    #[serde(rename = "status.info")]
    pub status_info: Option<String>,
    #[serde(rename = "status.success")]
    pub status_success: Option<String>,
    #[serde(rename = "status.warning")]
    pub status_warning: Option<String>,
    #[serde(rename = "tooltip.background")]
    pub tooltip_background: Option<String>,
    #[serde(rename = "tooltip.foreground")]
    pub tooltip_foreground: Option<String>,
    pub whitespace_foreground: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct BasicPalette {
    pub black: String,
    #[serde(rename = "black-dim")]
    pub black_dim: Option<String>,
    #[serde(rename = "black-bright")]
    pub black_bright: Option<String>,

    pub red: String,
    #[serde(rename = "red-dim")]
    pub red_dim: Option<String>,
    #[serde(rename = "red-bright")]
    pub red_bright: Option<String>,

    pub green: String,
    #[serde(rename = "green-dim")]
    pub green_dim: Option<String>,
    #[serde(rename = "green-bright")]
    pub green_bright: Option<String>,

    pub yellow: String,
    #[serde(rename = "yellow-dim")]
    pub yellow_dim: Option<String>,
    #[serde(rename = "yellow-bright")]
    pub yellow_bright: Option<String>,

    pub blue: String,
    #[serde(rename = "blue-dim")]
    pub blue_dim: Option<String>,
    #[serde(rename = "blue-bright")]
    pub blue_bright: Option<String>,

    pub magenta: String,
    #[serde(rename = "magenta-dim")]
    pub magenta_dim: Option<String>,
    #[serde(rename = "magenta-bright")]
    pub magenta_bright: Option<String>,

    pub cyan: String,
    #[serde(rename = "cyan-dim")]
    pub cyan_dim: Option<String>,
    #[serde(rename = "cyan-bright")]
    pub cyan_bright: Option<String>,

    pub white: String,
    #[serde(rename = "white-dim")]
    pub white_dim: Option<String>,
    #[serde(rename = "white-bright")]
    pub white_bright: Option<String>,

    pub orange: Option<String>,
    #[serde(rename = "orange-dim")]
    pub orange_dim: Option<String>,
    #[serde(rename = "orange-bright")]
    pub orange_bright: Option<String>,

    pub gray: Option<String>,
    #[serde(rename = "gray-dim")]
    pub gray_dim: Option<String>,
    #[serde(rename = "gray-bright")]
    pub gray_bright: Option<String>,

    pub brown: Option<String>,
    #[serde(rename = "brown-dim")]
    pub brown_dim: Option<String>,
    #[serde(rename = "brown-bright")]
    pub brown_bright: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BasicSyntax {
    pub comment: Option<String>,
    #[serde(rename = "comment.line")]
    pub comment_line: Option<String>,
    #[serde(rename = "comment.block")]
    pub comment_block: Option<String>,
    #[serde(rename = "invalid")]
    pub invalid: Option<String>,
    #[serde(rename = "invalid.deprecated")]
    pub invalid_deprecated: Option<String>,
    #[serde(rename = "invalid.illegal")]
    pub invalid_illegal: Option<String>,
    pub string: Option<String>,
    #[serde(rename = "string.quoted")]
    pub string_quoted: Option<String>,
    #[serde(rename = "string.single")]
    pub string_quoted_single: Option<String>,
    #[serde(rename = "string.double")]
    pub string_quoted_double: Option<String>,
    #[serde(rename = "string.regexp")]
    pub string_regexp: Option<String>,
    #[serde(rename = "string.template")]
    pub string_template: Option<String>,
    #[serde(rename = "string.interpolated")]
    pub string_interpolated: Option<String>,
    #[serde(rename = "string.unquoted")]
    pub string_unquoted: Option<String>,
    pub constant: Option<String>,
    #[serde(rename = "constant.numeric")]
    pub constant_numeric: Option<String>,
    #[serde(rename = "constant.numeric.integer")]
    pub constant_numeric_integer: Option<String>,
    #[serde(rename = "constant.numeric.float")]
    pub constant_numeric_float: Option<String>,
    #[serde(rename = "constant.numeric.hex")]
    pub constant_numeric_hex: Option<String>,
    #[serde(rename = "constant.numeric.exponential")]
    pub constant_numeric_exponential: Option<String>,
    #[serde(rename = "constant.language")]
    pub constant_language: Option<String>,
    #[serde(rename = "constant.language.boolean")]
    pub constant_language_boolean: Option<String>,
    #[serde(rename = "constant.other")]
    pub constant_other: Option<String>,
    #[serde(rename = "constant.character")]
    pub constant_character: Option<String>,
    #[serde(rename = "constant.character.escape")]
    pub constant_character_escape: Option<String>,
    #[serde(rename = "constant.character.entity")]
    pub constant_character_entity: Option<String>,
    #[serde(rename = "entity")]
    pub entity: Option<String>,
    #[serde(rename = "entity.name")]
    pub entity_name: Option<String>,
    #[serde(rename = "entity.name.class")]
    pub entity_name_class: Option<String>,
    #[serde(rename = "entity.name.filename")]
    pub entity_name_filename: Option<String>,
    #[serde(rename = "entity.name.function")]
    pub entity_name_function: Option<String>,
    #[serde(rename = "entity.name.tag")]
    pub entity_name_tag: Option<String>,
    #[serde(rename = "entity.name.variable")]
    pub entity_name_variable: Option<String>,
    #[serde(rename = "entity.name.type")]
    pub entity_name_type: Option<String>,
    #[serde(rename = "entity.name.namespace")]
    pub entity_name_namespace: Option<String>,
    #[serde(rename = "entity.name.section")]
    pub entity_name_section: Option<String>,
    #[serde(rename = "entity.other")]
    pub entity_other: Option<String>,
    #[serde(rename = "entity.other.attribute-name")]
    pub entity_other_attribute_name: Option<String>,
    #[serde(rename = "entity.other.inherited-class")]
    pub entity_other_inherited_class: Option<String>,
    pub keyword: Option<String>,
    #[serde(rename = "keyword.control")]
    pub keyword_control: Option<String>,
    #[serde(rename = "keyword.declaration")]
    pub keyword_declaration: Option<String>,
    #[serde(rename = "keyword.operator")]
    pub keyword_operator: Option<String>,
    #[serde(rename = "keyword.other")]
    pub keyword_other: Option<String>,
    #[serde(rename = "storage")]
    pub storage: Option<String>,
    #[serde(rename = "storage.type")]
    pub storage_type: Option<String>,
    #[serde(rename = "storage.modifier")]
    pub storage_modifier: Option<String>,
    #[serde(rename = "support")]
    pub support: Option<String>,
    #[serde(rename = "support.function")]
    pub support_function: Option<String>,
    #[serde(rename = "support.class")]
    pub support_class: Option<String>,
    #[serde(rename = "support.type")]
    pub support_type: Option<String>,
    #[serde(rename = "support.constant")]
    pub support_constant: Option<String>,
    #[serde(rename = "support.variable")]
    pub support_variable: Option<String>,
    #[serde(rename = "variable")]
    pub variable: Option<String>,
    #[serde(rename = "variable.parameter")]
    pub variable_parameter: Option<String>,
    #[serde(rename = "variable.language")]
    pub variable_language: Option<String>,
    #[serde(rename = "variable.function")]
    pub variable_function: Option<String>,
    #[serde(rename = "punctuation")]
    pub punctuation: Option<String>,
    #[serde(rename = "punctuation.accessor")]
    pub punctuation_accessor: Option<String>,
    #[serde(rename = "punctuation.section")]
    pub punctuation_section: Option<String>,
    #[serde(rename = "punctuation.separator")]
    pub punctuation_separator: Option<String>,
    #[serde(rename = "punctuation.definition")]
    pub punctuation_definition: Option<String>,
    #[serde(rename = "punctuation.definition.string")]
    pub punctuation_definition_string: Option<String>,
    #[serde(rename = "punctuation.terminator")]
    pub punctuation_terminator: Option<String>,
    pub markup: Option<String>,
    #[serde(rename = "markup.heading")]
    pub markup_heading: Option<String>,
    #[serde(rename = "markup.bold")]
    pub markup_bold: Option<String>,
    #[serde(rename = "markup.code")]
    pub markup_code: Option<String>,
    #[serde(rename = "markup.italic")]
    pub markup_italic: Option<String>,
    #[serde(rename = "markup.quote")]
    pub markup_quote: Option<String>,
    #[serde(rename = "markup.underline")]
    pub markup_underline: Option<String>,
    #[serde(rename = "markup.list")]
    pub markup_list: Option<String>,
    #[serde(rename = "markup.link")]
    pub markup_link: Option<String>,
    #[serde(rename = "markup.raw")]
    pub markup_raw: Option<String>,
    #[serde(rename = "diff")]
    pub diff: Option<String>,
    #[serde(rename = "diff.added")]
    pub diff_added: Option<String>,
    #[serde(rename = "diff.changed")]
    pub diff_changed: Option<String>,
    #[serde(rename = "diff.deleted")]
    pub diff_deleted: Option<String>,
}

impl BasicPalette {
    pub fn to_normal_vec(&self) -> Vec<(String, String)> {
        vec![
            ("black".to_string(), self.black.clone()),
            ("red".to_string(), self.red.clone()),
            ("green".to_string(), self.green.clone()),
            ("yellow".to_string(), self.yellow.clone()),
            ("blue".to_string(), self.blue.clone()),
            ("magenta".to_string(), self.magenta.clone()),
            ("cyan".to_string(), self.cyan.clone()),
            ("white".to_string(), self.white.clone()),
        ]
    }
}

impl fmt::Display for BasicPalette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let palette_vec: Vec<(String, String)> = self
            .to_normal_vec()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        for (key, value) in palette_vec {
            writeln!(f, "  {key}: \"{value}\"")?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub system: SchemeSystem,
    #[serde(rename = "system-version")]
    pub system_version: String,
    pub author: String,

    pub name: Option<String>,
    #[serde(rename = "theme-author")]
    pub theme_author: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
}
