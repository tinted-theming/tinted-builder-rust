mod base16;

use crate::{error::TintedBuilderError, scheme::Scheme};

/// A struct representing a template that can be rendered with the provided color scheme.
///
/// The `Template` struct holds the content of the template and the scheme used to render it. It
/// provides methods to create a new template and render it into a `String` using the specified
/// color scheme.
pub struct Template {
    content: String,
    scheme: Scheme,
}

impl Template {
    /// Creates a new `Template` instance.
    ///
    /// # Arguments
    ///
    /// * `content` - A `String` representing the content of the template.
    /// * `scheme` - A `Scheme` enum that determines which color scheme to use when rendering the template.
    ///
    /// # Returns
    ///
    /// A new `Template` instance with the provided content and scheme.
    pub fn new(content: String, scheme: Scheme) -> Template {
        Template { content, scheme }
    }

    /// Renders the template into a `String` using the provided color scheme.
    ///
    /// This method applies the specified `Scheme` to the template content, converting placeholders
    /// in the content to their corresponding values from the scheme context.
    ///
    /// # Errors
    ///
    /// Returns a `TintedBuilderError` if the rendering process fails. This could happen if the
    /// content contains placeholders that cannot be resolved using the scheme context.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinted_builder::{Template, Scheme};
    ///
    /// let scheme_yaml = r#"
    /// system: "base16"
    /// name: "Some name"
    /// author: "Some author"
    /// variant: "dark"
    /// palette:
    ///   base00: "241b26"
    ///   base01: "2f2a3f"
    ///   base02: "46354a"
    ///   base03: "6c3cb2"
    ///   base04: "7e5f83"
    ///   base05: "eed5d9"
    ///   base06: "d9c2c6"
    ///   base07: "e4ccd0"
    ///   base08: "877bb6"
    ///   base09: "de5b44"
    ///   base0A: "a84a73"
    ///   base0B: "c965bf"
    ///   base0C: "9c5fce"
    ///   base0D: "6a9eb5"
    ///   base0E: "78a38f"
    ///   base0F: "a3a079"
    /// "#;
    /// let template = Template::new(
    ///     r#"{{scheme-system}} scheme name is "{{scheme-name}}" and first color is #{{base00-hex}}"#.to_string(),
    ///     Scheme::Base16(serde_yaml::from_str(scheme_yaml).unwrap())
    /// );
    /// let rendered = template.render().unwrap();
    ///
    /// assert_eq!(
    ///     rendered,
    ///     r#"base16 scheme name is "Some name" and first color is #241b26"#
    /// );
    /// ```
    pub fn render(&self) -> Result<String, TintedBuilderError> {
        match self.scheme {
            Scheme::Base16(ref scheme) | Scheme::Base24(ref scheme) => {
                let ctx = base16::to_template_context(&scheme.clone());
                let rendered = base16::render(&self.content, &ctx)?;

                Ok(rendered)
            }
        }
    }
}
