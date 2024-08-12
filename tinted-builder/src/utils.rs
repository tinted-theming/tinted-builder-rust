use regex::Regex;
use std::collections::HashMap;

pub(crate) fn slugify(input: &str) -> String {
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
    .cloned()
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
