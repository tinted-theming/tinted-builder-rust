use anyhow::{anyhow, Context, Result};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use crate::scheme::SchemeType;

pub enum TemplateContent {
    Yaml(String),
    TmTheme(String),
}

pub struct Template {
    content: TemplateContent,
}

pub(crate) fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        remove_file(path).with_context(|| format!("Unable to remove file: {}", path.display()))?;
    }

    let mut file =
        File::create(path).with_context(|| format!("Unable to create file: {}", path.display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
// background: Option<String>,
// foreground: Option<String>,
// caret: Option<String>,
// invisibles: Option<String>,
// lineHighlight: Option<String>,
// selection: Option<String>,
// selectionBorder: Option<String>,
// findHighlight: Option<String>,
// findHighlightForeground: Option<String>,
// activeGuide: Option<String>,
// bracketsForeground: Option<String>,
// bracketContentsForeground: Option<String>,
// highlight: Option<String>,
// guide: Option<String>,
// stackGuide: Option<String>,
// underline: Option<bool>,
// tagsOptions: Option<BracketOptions>,
// bracketsOptions: Option<BracketOptions>,
// bracketsContentOptions: Option<BracketOptions>,

impl Template {
    pub fn new(content: TemplateContent) -> Result<Template> {
        Ok(Template { content })
    }

    fn to_template_context(scheme_type: &SchemeType) -> HashMap<String, String> {
        match scheme_type {
            SchemeType::Yaml(scheme) => {
                let mut context = HashMap::new();

                context.insert("scheme-name".to_string(), scheme.name.clone());
                context.insert("scheme-author".to_string(), scheme.author.clone());
                context.insert(
                    "scheme-description".to_string(),
                    scheme.description.clone().unwrap_or_default(),
                );
                context.insert("scheme-slug".to_string(), scheme.slug.clone());
                context.insert(
                    "scheme-slug-underscored".to_string(),
                    scheme.slug.replace('-', "_"),
                );
                context.insert("scheme-system".to_string(), scheme.system.clone());
                context.insert("scheme-variant".to_string(), scheme.variant.clone());
                context.insert(
                    format!("scheme-is-{}-variant", scheme.variant),
                    "true".to_string(),
                );

                for (name, color) in scheme.palette.iter() {
                    let hex = color.hex.clone();
                    let rgb = color.rgb;

                    context.insert(
                        format!("{}-hex", name),
                        format!("{}{}{}", color.hex.0, color.hex.1, color.hex.2),
                    );
                    context.insert(
                        format!("{}-hex-bgr", name),
                        format!("{}{}{}", color.hex.2, color.hex.1, color.hex.0),
                    );
                    context.insert(format!("{}-hex-r", name), hex.0);
                    context.insert(format!("{}-hex-g", name), hex.1);
                    context.insert(format!("{}-hex-b", name), hex.2);
                    context.insert(format!("{}-rgb-r", name), rgb.0.to_string());
                    context.insert(format!("{}-rgb-g", name), rgb.1.to_string());
                    context.insert(format!("{}-rgb-b", name), rgb.2.to_string());
                    context.insert(format!("{}-dec-r", name), (rgb.0 / 255).to_string());
                    context.insert(format!("{}-dec-g", name), (rgb.1 / 255).to_string());
                    context.insert(format!("{}-dec-b", name), (rgb.2 / 255).to_string());
                }

                context
            }
            SchemeType::TmTheme(scheme) => HashMap::default(),
        }
    }

    pub fn render(&self, scheme_type: &SchemeType) -> Result<String> {
        match (&self.content, scheme_type) {
            (TemplateContent::Yaml(content), SchemeType::Yaml(_)) => {
                let context = serde_yaml::to_string(&Self::to_template_context(scheme_type))?;
                let rendered = ribboncurls::render(content, &context, None)?;

                Ok(rendered)
            }
            (TemplateContent::TmTheme(content), SchemeType::TmTheme(scheme)) => {
                let mut reader = Reader::from_str(content);
                reader.config_mut().trim_text(true);

                let mut buf: Vec<u8> = Vec::new();

                // Variables to hold temporary data
                let mut current_key = String::new();
                let mut current_value = String::new();
                let mut current_settings = HashMap::new();
                let mut theme_name = String::new();
                let mut uuid = String::new();
                let mut settings = Vec::new();
                let mut in_settings_dict = false;
                let mut current_setting_name = None;
                let mut current_scope = None;

                // Read the XML events
                loop {
                    match reader.read_event()? {
                        Event::Start(ref e) => match e.name() {
                            QName(b"key") => {
                                current_key.clear();
                            }
                            QName(b"string") => {
                                current_value.clear();
                            }
                            QName(b"array") => {
                                if !in_settings_dict {
                                    in_settings_dict = true;
                                }
                            }
                            QName(b"dict") => {
                                if in_settings_dict {
                                    // New settings dict inside an array
                                    if !current_settings.is_empty() {
                                        settings.push(Setting {
                                            name: current_setting_name.take(),
                                            scope: current_scope.take(),
                                            settings: current_settings.clone(),
                                        });
                                        current_settings.clear();
                                    }
                                }
                            }
                            _ => (),
                        },
                        Event::End(ref e) => match e.name() {
                            QName(b"key") => {
                                if current_key == "name" && !in_settings_dict {
                                    theme_name = current_value.clone();
                                } else if current_key == "uuid" && !in_settings_dict {
                                    uuid = current_value.clone();
                                } else if current_key == "name" {
                                    current_setting_name = Some(current_value.clone());
                                } else if current_key == "scope" {
                                    current_scope = Some(current_value.clone());
                                }
                            }
                            QName(b"string") => {
                                current_settings.insert(current_key.clone(), current_value.clone());
                            }
                            QName(b"dict") => {
                                if in_settings_dict && !current_settings.is_empty() {
                                    settings.push(Setting {
                                        name: current_setting_name.take(),
                                        scope: current_scope.take(),
                                        settings: current_settings.clone(),
                                    });
                                    current_settings.clear();
                                }
                            }
                            QName(b"array") => {
                                in_settings_dict = false;
                            }
                            _ => (),
                        },
                        Event::Text(e) => {
                            let text = e.unescape()?.into_owned();
                            if current_key.is_empty() {
                                current_key = text;
                            } else {
                                current_value = text;
                            }
                        }
                        Event::Eof => break,
                        _ => (),
                    }

                    buf.clear();
                }

                // Create the Theme struct
                let theme = Theme {
                    name: theme_name,
                    uuid,
                    settings,
                };

                // Print the parsed theme
                // let content: Plist = quick_xml::reader::Reader::from_str(content);
                // dbg!(content);
                // let context = serde_yaml::to_string(&Self::to_template_context(scheme_type))?;
                // let rendered = ribboncurls::render(content, &context, None)?;

                // Ok(rendered)
                // let context = serde_yaml::to_string(&Self::to_template_context(scheme))?;
                // let rendered = ribboncurls::render(&self.content, &context, None)?;
                //
                dbg!(theme);

                Ok(String::from("hello"))
            }
            (_, _) => Err(anyhow!("Mismatch between template type and scheme type")),
        }
    }

    #[deprecated(
        since = "0.4.0",
        note = "Please use the `render` method instead and write the output to a file yourself."
    )]
    pub fn render_to_file(&self, output_path: &Path, scheme_type: &SchemeType) -> Result<&Self> {
        match (&self.content, scheme_type) {
            (TemplateContent::Yaml(content), SchemeType::Yaml(scheme)) => {
                let context = serde_yaml::to_string(&Self::to_template_context(scheme_type))?;
                let rendered = ribboncurls::render(content, &context, None)?;

                write_to_file(output_path, &rendered)?;

                Ok(self)
            }
            (TemplateContent::TmTheme(content), SchemeType::TmTheme(scheme)) => Ok(self),
            (_, _) => Err(anyhow!("Mismatch between template type and scheme type")),
        }
    }
}

#[derive(Debug)]
struct Theme {
    name: String,
    uuid: String,
    settings: Vec<Setting>,
}

#[derive(Debug)]
struct Setting {
    name: Option<String>,
    scope: Option<String>,
    settings: HashMap<String, String>,
}

// #[derive(Debug, Deserialize)]
// struct Plist {
//     #[serde(rename = "dict")]
//     dict: Dict,
// }

// #[derive(Debug, Deserialize)]
// struct Dict {
//     #[serde(rename = "$value")]
//     elements: Vec<DictElement>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum DictElement {
//     Key(Key),
//     String(Value),
//     Array(Array),
//     Dict(NestedDict),
//     Other(OtherElement), // Add a catch-all variant
// }

// #[derive(Debug, Deserialize)]
// struct Key {
//     #[serde(rename = "key")]
//     key: String,
// }

// #[derive(Debug, Deserialize)]
// struct Value {
//     #[serde(rename = "string")]
//     value: String,
// }

// #[derive(Debug, Deserialize)]
// struct Array {
//     #[serde(rename = "array")]
//     array: Vec<Dict>,
// }

// #[derive(Debug, Deserialize)]
// struct NestedDict {
//     #[serde(rename = "dict")]
//     dict: Dict,
// }

// #[derive(Debug, Deserialize)]
// struct OtherElement {
//     #[serde(rename = "$value")]
//     content: Vec<OtherContent>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum OtherContent {
//     Key(Key),
//     String(Value),
//     Array(Array),
//     Dict(NestedDict),
// }

// impl fmt::Display for OtherContent {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             OtherContent::Key(k) => write!(f, "Key: {}", k.key),
//             OtherContent::String(v) => write!(f, "String: {}", v.value),
//             OtherContent::Array(a) => write!(f, "Array with {} items", a.array.len()),
//             OtherContent::Dict(d) => write!(f, "Nested Dict"),
//         }
//     }
// }

// impl Dict {
//     fn get_key_value(&self) -> Vec<(String, String)> {
//         let mut result = Vec::new();
//         let mut key_opt: Option<String> = None;

//         for element in &self.elements {
//             match element {
//                 DictElement::Key(key) => key_opt = Some(key.key.clone()),
//                 DictElement::String(value) => {
//                     if let Some(key) = key_opt.take() {
//                         result.push((key, value.value.clone()));
//                     }
//                 }
//                 _ => {} // Ignore other elements for key-value extraction
//             }
//         }

//         result
//     }
// }
// ------------

// #[derive(Debug, Deserialize)]
// struct Plist {
//     #[serde(rename = "dict")]
//     dict: Dict,
// }

// #[derive(Debug, Deserialize)]
// struct Dict {
//     #[serde(rename = "key")]
//     keys: Vec<String>,
//     #[serde(rename = "string")]
//     strings: Vec<String>,
//     #[serde(rename = "array")]
//     array: Option<SettingsArray>,
// }

// #[derive(Debug, Deserialize)]
// struct SettingsArray {
//     #[serde(rename = "dict")]
//     dicts: Vec<SettingsDict>,
// }

// #[derive(Debug, Deserialize)]
// struct SettingsDict {
//     #[serde(rename = "key")]
//     keys: Vec<String>,
//     #[serde(rename = "string")]
//     strings: Vec<String>,
//     #[serde(rename = "dict")]
//     settings: Option<Box<InnerSettings>>,
// }

// #[derive(Debug, Deserialize)]
// struct InnerSettings {
//     #[serde(rename = "key")]
//     keys: Vec<String>,
//     #[serde(rename = "string")]
//     strings: Vec<String>,
//     #[serde(rename = "dict")]
//     dicts: Option<Vec<Box<InnerSettings>>>,
// }

// -----------------

// #[derive(Debug, Deserialize)]
// struct Plist {
//     #[serde(rename = "dict")]
//     value: HashMap<String, XmlValue>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum XmlValue {
//     String(String),
//     Array(Vec<HashMap<String, XmlValue>>),
//     Dict(HashMap<String, XmlValue>),
// }
