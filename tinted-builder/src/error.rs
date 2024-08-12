use ribboncurls::RibboncurlsError;
use thiserror::Error;

/// An error type representing the various errors that can occur when using tinted-builder
///
/// This error type is non-exhaustive, meaning additional variants may be added in future versions without
/// it being considered a breaking change. The enum variants cover a range of possible issues that might
/// arise during the processing of color schemes, including missing properties, deserialization errors,
/// and rendering issues.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum TintedBuilderError {
    /// Error indicating that a required property in the scheme is missing.
    ///
    /// This variant is used when a necessary property for the scheme's configuration is not found.
    #[error("missing scheme property: {0}")]
    SchemeMissingProperty(String),

    /// Error that occurs when YAML deserialization fails.
    ///
    /// This variant wraps the `serde_yaml::Error` and is used when there is an issue converting
    /// a YAML string into the corresponding Rust data structure.
    #[error("unable to deserialize yaml")]
    YamlDeserialize(#[from] serde_yaml::Error),

    /// Error that occurs during rendering using Ribboncurls.
    ///
    /// This variant wraps the `RibboncurlsError` and is used when an error is encountered while
    /// rendering a template or other content using Ribboncurls.
    #[error("unable to render")]
    RibboncurlsRender(#[from] RibboncurlsError),

    /// Error that occurs when a string slice cannot be converted to an integer with the given base.
    ///
    /// This variant wraps `std::num::ParseIntError` and is used when a string slice, such as a color
    /// value in hexadecimal format, fails to parse correctly.
    #[error("unable to convert string slice to integer with given base")]
    ColorRadix(#[from] std::num::ParseIntError),

    /// Error indicating that a hex color input is not formatted correctly.
    ///
    /// This variant is used when a color string that is expected to be in hex format does not match
    /// the expected format.
    #[error("hex input is not formatted correctly")]
    HexInputFormat,

    /// Error indicating that an invalid scheme variant was provided.
    ///
    /// This variant is used when an input string does not correspond to any valid scheme variant,
    /// such as "dark" or "light".
    #[error("invalid scheme variant: {0}")]
    InvalidSchemeVariant(String),
}
