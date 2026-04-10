use crate::scheme::SchemeVariant;
use crate::{scheme::tinted8::SchemeSystem, SchemeSupports};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{Mapping, Value};
use std::fmt;

#[derive(Serialize)]
pub struct Tinted8Scheme {
    pub scheme: Meta,
    pub palette: BasicPalette,
    pub syntax: Option<BasicSyntax>,
    pub ui: Option<BasicUi>,
    pub variant: SchemeVariant,
}

// Helper type that mirrors `Tinted8Scheme` for inner deserialization.
#[derive(Deserialize)]
struct Tinted8SchemeHelper {
    pub scheme: Meta,
    pub palette: BasicPalette,
    pub syntax: Option<BasicSyntax>,
    pub ui: Option<BasicUi>,
    pub variant: SchemeVariant,
}

impl From<Tinted8SchemeHelper> for Tinted8Scheme {
    fn from(h: Tinted8SchemeHelper) -> Self {
        Self {
            scheme: h.scheme,
            palette: h.palette,
            syntax: h.syntax,
            ui: h.ui,
            variant: h.variant,
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
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct BasicUi {
    #[serde(rename = "global.background.normal")]
    pub global_background_normal: Option<String>,
    #[serde(rename = "global.background.dark")]
    pub global_background_dark: Option<String>,
    #[serde(rename = "global.background.light")]
    pub global_background_light: Option<String>,
    pub deprecated: Option<String>,
    #[serde(rename = "accent")]
    pub accent: Option<String>,
    #[serde(rename = "border")]
    pub border: Option<String>,
    #[serde(rename = "cursor.normal")]
    pub cursor_normal: Option<String>,
    #[serde(rename = "cursor.muted")]
    pub cursor_muted: Option<String>,
    #[serde(rename = "global.foreground.normal")]
    pub global_foreground_normal: Option<String>,
    #[serde(rename = "global.foreground.dark")]
    pub global_foreground_dark: Option<String>,
    #[serde(rename = "global.foreground.light")]
    pub global_foreground_light: Option<String>,
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
    #[serde(rename = "highlight.button.background")]
    pub highlight_button_background: Option<String>,
    #[serde(rename = "highlight.button.foreground")]
    pub highlight_button_foreground: Option<String>,
    #[serde(rename = "indent-guide.background")]
    pub indent_guide_background: Option<String>,
    #[serde(rename = "indent-guide.active-background")]
    pub indent_guide_active_background: Option<String>,
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
    #[serde(rename = "whitespace.foreground")]
    pub whitespace_foreground: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

pub use crate::scheme::tinted8::structure::syntax::BasicSyntax;

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
    pub supports: SchemeSupports,
    pub author: String,
    pub name: Option<String>,
    #[serde(rename = "theme-author")]
    pub theme_author: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub style: Option<String>,
}
