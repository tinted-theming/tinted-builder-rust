pub mod meta;
pub mod palette;
pub mod syntax;
pub mod ui;

pub use crate::scheme::tinted8::structure::meta::SchemeMeta;
pub use crate::scheme::tinted8::structure::palette::Palette;
pub use crate::scheme::tinted8::structure::syntax::Syntax;
pub use crate::scheme::tinted8::structure::ui::Ui;
use crate::scheme::tinted8::yaml::Tinted8Scheme as YamlTinted8Scheme;
use crate::tinted8::SUPPORTED_BUILDER_SPEC_VERSION;
use crate::utils::slugify;
use crate::utils::titlecasify;
use crate::SchemeVariant;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// Fully resolved Tinted8 scheme used by templates and downstream tooling.
///
/// This structure is created from YAML via `tinted8::yaml`, deriving missing variants and
/// supplemental colors according to the Tinted8 builder rules.
#[derive(Debug, Clone, Serialize)]
pub struct Scheme {
    pub scheme: SchemeMeta,
    pub variant: SchemeVariant,
    pub family: Option<String>,
    pub style: Option<String>,
    pub palette: Palette,
    pub syntax: Syntax,
    pub ui: Ui,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "scheme:")?;
        writeln!(f, "  system: \"{}\"", self.scheme.system)?;
        writeln!(
            f,
            "  supported-builder-version: \"{}\"",
            self.scheme.supported_builder_version
        )?;
        writeln!(
            f,
            "  supported-styling-version: \"{}\"",
            self.scheme.supported_styling_version
        )?;
        writeln!(f, "  name: \"{}\"", self.scheme.name)?;
        writeln!(f, "  author: \"{}\"", self.scheme.author)?;
        writeln!(f, "  theme-author: \"{}\"", self.scheme.theme_author)?;
        writeln!(f, "  slug: \"{}\"", self.scheme.slug)?;
        if let Some(ref desc) = self.scheme.description {
            writeln!(f, "  description: \"{desc}\"")?;
        }
        #[allow(clippy::writeln_empty_string)]
        writeln!(f, "")?;
        writeln!(f, "variant: \"{}\"", self.variant)?;
        if let Some(ref family) = self.family {
            writeln!(f, "family: \"{family}\"")?;
        }
        if let Some(ref style) = self.style {
            writeln!(f, "style: \"{style}\"")?;
        }

        #[allow(clippy::writeln_empty_string)]
        writeln!(f, "")?;

        writeln!(f, "palette:")?;

        #[allow(clippy::writeln_empty_string)]
        writeln!(f, "")?;

        writeln!(f, "{}", self.palette)?;

        writeln!(f, "syntax:")?;
        writeln!(f, "{}", self.syntax)?;

        writeln!(f, "ui:")?;
        writeln!(f, "{}", self.ui)?;

        Ok(())
    }
}

impl<'de> Deserialize<'de> for Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = YamlTinted8Scheme::deserialize(deserializer)?;

        let (name, slug): (String, String) = match (
            &wrapper.scheme.name,
            &wrapper.scheme.slug,
            &wrapper.family,
            &wrapper.style,
        ) {
            (Some(name), Some(slug), _, _) => (name.to_owned(), slug.to_owned()),
            (Some(name), None, _, _) => (name.to_owned(), slugify(name)),
            (None, Some(slug), _, _) => (titlecasify(slug), slug.to_owned()),
            (None, None, Some(family), Some(style)) => {
                let name = format!("{family}-{style}");

                (name.clone(), slugify(&name))
            }
            (None, None, Some(family), None) => {
                let name = family.clone();

                (name.clone(), slugify(&name))
            }
            _ => {
                return Err(serde::de::Error::custom(
                    "Either 'name', 'slug' or 'family' must exist in yaml scheme",
                ))
            }
        };

        let palette =
            Palette::try_from_basic(&wrapper.palette).map_err(serde::de::Error::custom)?;
        let ui = Ui::try_from_basic(wrapper.ui.unwrap_or_default(), &palette)
            .map_err(serde::de::Error::custom)?;
        let syntax = Syntax::try_from_basic(&wrapper.syntax.unwrap_or_default(), &palette)
            .map_err(serde::de::Error::custom)?;
        let builder_spec_version = SUPPORTED_BUILDER_SPEC_VERSION.to_string();
        let styling_spec_version = wrapper.scheme.system_version.clone();
        let scheme_meta = SchemeMeta {
            name,
            slug,
            system: wrapper.scheme.system,
            author: wrapper.scheme.author.clone(),
            description: wrapper.scheme.description,
            theme_author: wrapper.scheme.theme_author.unwrap_or(wrapper.scheme.author),
            supported_builder_version: builder_spec_version,
            supported_styling_version: styling_spec_version,
        };

        Ok(Self {
            scheme: scheme_meta,
            family: wrapper.family,
            style: wrapper.style,
            variant: wrapper.variant,
            syntax,
            ui,
            palette,
        })
    }
}
