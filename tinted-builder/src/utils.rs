use regex::Regex;
use std::collections::HashMap;

use crate::{Color, TintedBuilderError};

/// Slugifies a string using ASCII-only, kebab-case output.
///
/// Examples:
/// - "Catppuccin Mocha" -> "catppuccin-mocha"
/// - "Rosé Pine" -> "rose-pine"
/// - "Default (Dark)" -> "default-dark"
pub fn slugify(input: &str) -> String {
    let char_map: HashMap<char, &str> = [
        ('á', "a"),
        ('à', "a"),
        ('â', "a"),
        ('ä', "a"),
        ('ã', "a"),
        ('å', "a"),
        ('æ', "ae"),
        ('ç', "c"),
        ('é', "e"),
        ('è', "e"),
        ('ê', "e"),
        ('ë', "e"),
        ('í', "i"),
        ('ì', "i"),
        ('î', "i"),
        ('ï', "i"),
        ('ł', "l"),
        ('ñ', "n"),
        ('ń', "n"),
        ('ó', "o"),
        ('ò', "o"),
        ('ô', "o"),
        ('ö', "o"),
        ('õ', "o"),
        ('ø', "o"),
        ('œ', "oe"),
        ('ś', "s"),
        ('ú', "u"),
        ('ù', "u"),
        ('û', "u"),
        ('ü', "u"),
        ('ý', "y"),
        ('ÿ', "y"),
        ('ż', "z"),
        ('ź', "z"),
        ('š', "s"),
        ('č', "c"),
        ('ř', "r"),
        ('đ', "d"),
        ('ß', "ss"),
        ('þ', "th"),
        ('ħ', "h"),
    ]
    .iter()
    .copied()
    .collect();

    let mut slug = String::new();
    for c in input.to_lowercase().chars() {
        match c {
            'a'..='z' | '0'..='9' => slug.push(c),
            ' ' | '-' | '_' => slug.push('-'),
            _ => {
                if let Some(replacement) = char_map.get(&c) {
                    slug.push_str(replacement);
                }
            }
        }
    }

    #[allow(clippy::expect_used)]
    let re = Regex::new(r"-+").expect("Unable to unwrap regex");
    let cleaned_slug = re.replace_all(&slug, "-").to_string();

    cleaned_slug.trim_matches('-').to_string()
}

/// Converts a slug (kebab/underscore) to Title Case words.
///
/// Examples:
/// - "catppuccin-mocha" -> "Catppuccin Mocha"
/// - "`rose_pine`" -> "Rose Pine"
pub fn titlecasify(input: &str) -> String {
    #[allow(clippy::expect_used)]
    let sep = Regex::new(r"[-_]+").expect("Unable to unwrap regex");
    let replaced = sep.replace_all(input.trim(), " ");

    replaced
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                let mut s = String::new();
                s.extend(first.to_uppercase());
                s.push_str(chars.as_str());
                s
            })
        })
        .collect::<Vec<String>>()
        .join(" ")
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
pub fn parse_or_inherit(
    value_list: &[Option<&str>],
    default: &Color,
) -> Result<Color, TintedBuilderError> {
    let value_list: Vec<String> = value_list
        .iter()
        .filter_map(|s| s.map(std::string::ToString::to_string))
        .collect();

    value_list.first().map_or_else(
        || Ok(default.clone()),
        |val| {
            Color::new(val, None, None)
                .map_err(|e| TintedBuilderError::UnableToConvertFrom(e.to_string()))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{slugify, titlecasify};

    #[test]
    fn slugify_basic_and_unicode() {
        assert_eq!(slugify("Catppuccin Mocha"), "catppuccin-mocha");
        assert_eq!(slugify("Rosé Pine"), "rose-pine");
        assert_eq!(slugify("Default (Dark)"), "default-dark");
        assert_eq!(slugify("  Trim  Spaces  "), "trim-spaces");
    }

    #[test]
    fn titlecasify_basic() {
        assert_eq!(titlecasify("catppuccin-mocha"), "Catppuccin Mocha");
        assert_eq!(titlecasify("rose-pine"), "Rose Pine");
        assert_eq!(titlecasify("default-dark"), "Default Dark");
    }
}
