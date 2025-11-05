use regex::Regex;
use std::collections::HashMap;

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
