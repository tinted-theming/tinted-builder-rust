use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tmtheme_deserialize::{deserialize, SettingValue};

const REQUIRED_META_KEYS: &[&str] = &["system", "variant", "name", "author"];

pub fn to_template_context(content: &str) -> Result<Settings> {
    let context = deserialize(content)?;
    let meta_context = context
        .get("meta")
        .ok_or_else(|| anyhow!("`meta` property missing from tmTheme hashmap"))?;

    match meta_context.as_ref() {
        SettingValue::HashMap(meta) => {
            for required_key in REQUIRED_META_KEYS {
                if !meta.contains_key(required_key.to_string().as_str()) {
                    return Err(anyhow!(
                        "TmTheme is missing required meta information key: {}",
                        required_key
                    ));
                }
            }
        }

        _ => return Err(anyhow!("The meta property should be a hashmap")),
    }

    Ok(context)
}

pub fn render(content: &str, tmscheme: &Settings) -> Result<String> {
    let yaml_context = serde_yaml::to_string(&tmscheme)?;
    let rendered = ribboncurls::render(content, &yaml_context, None)?;

    Ok(rendered)
}
