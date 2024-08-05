use anyhow::{anyhow, Context, Result};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
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
                let mut theme_properties = HashMap::new();
                let global_settings: RefCell<Vec<Setting>> = RefCell::new(Vec::new());
                let local_settings: RefCell<Vec<Setting>> = RefCell::new(Vec::new());
                let mut settings = &global_settings;
                let mut in_root_dict = false;
                let mut in_global_dict = false; // First item is global
                let mut in_settings_dict = false;
                let mut current_setting_name = None;
                let mut current_scope = None;
                let mut dict_depth = 0;
                let mut settings_dict_depth = 0;
                let global_dict_depth = 3;

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
                                current_key.clear();
                                if !in_root_dict {
                                    in_root_dict = true;
                                    in_global_dict = true;
                                }
                            }
                            QName(b"dict") => {
                                dict_depth += 1;

                                // if current_key == "settings" {
                                //     settings_dict_depth = dict_depth;
                                // }

                                // Don't do anything for root dict
                                if in_root_dict {
                                    if !in_settings_dict {
                                        in_settings_dict = true;
                                    }

                                    if !current_settings.is_empty() {
                                        // Push the current settings to the settings vector
                                        settings.borrow_mut().push(Setting {
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
                                // Handle key based on context
                            }
                            QName(b"string") => {
                                dbg!(&current_key);

                                if current_key == "name" {
                                    current_setting_name = Some(current_value.clone());
                                }

                                if current_key == "scope" {
                                    current_scope = Some(current_value.clone());
                                }

                                if in_root_dict {
                                    dbg!(&current_key);
                                    dbg!(&current_value);
                                    current_settings
                                        .insert(current_key.clone(), current_value.clone());
                                } else if current_key == "name" || current_key == "uuid" {
                                    theme_properties
                                        .insert(current_key.clone(), current_value.clone());
                                }
                            }
                            QName(b"dict") => {
                                // dbg!("last");
                                // dbg!(&current_settings);
                                if !current_settings.is_empty() {
                                    // Push the current settings to the settings vector
                                    dbg!(&current_setting_name);
                                    settings.borrow_mut().push(Setting {
                                        name: current_setting_name.take(),
                                        scope: current_scope.take(),
                                        settings: current_settings.clone(),
                                    });
                                    current_settings.clear();
                                }

                                dict_depth -= 1;

                                if in_global_dict && dict_depth == (global_dict_depth - 1) {
                                    in_global_dict = false;
                                    settings = &local_settings;
                                }
                            }
                            QName(b"array") => {
                                in_root_dict = false;
                            }
                            _ => (),
                        },
                        Event::Text(e) => {
                            // println!("{:?}", e);
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
                let global_settings: Vec<Box<Setting>> = (*(global_settings.borrow()))
                    .iter()
                    .cloned()
                    .map(Box::new)
                    .collect();
                let local_settings: Vec<Box<Setting>> = (*(local_settings.borrow()))
                    .iter()
                    .cloned()
                    .map(Box::new)
                    .collect();
                let theme = Theme {
                    properties: theme_properties,
                    global_settings,
                    settings: local_settings,
                };

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

#[derive(Clone, Debug)]
struct Theme {
    properties: HashMap<String, String>,
    global_settings: Vec<Box<Setting>>,
    settings: Vec<Box<Setting>>,
}

#[derive(Clone, Debug)]
struct Setting {
    name: Option<String>,
    scope: Option<String>,
    settings: HashMap<String, String>,
}
