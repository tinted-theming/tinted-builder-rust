use tinted_builder::{tinted8::Scheme as Tinted8Scheme, Scheme, TintedBuilderError};

#[test]
fn deserialize_minimal_scheme() -> Result<(), TintedBuilderError> {
    let ts: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert_eq!(ts.scheme.name, "Test Scheme");
    assert_eq!(ts.scheme.slug, "test-scheme");
    assert_eq!(ts.scheme.author, "Test Author <test@example.com>");
    assert_eq!(ts.palette.black_normal.to_hex(), "131721");
    assert_eq!(ts.palette.red_normal.to_hex(), "f07178");

    Ok(())
}

#[test]
fn deserialize_family_style_derives_name() -> Result<(), TintedBuilderError> {
    let ts: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_FAMILY_STYLE)?;

    assert_eq!(ts.scheme.name, "Ayu-Mirage");
    assert_eq!(ts.scheme.slug, "ayu-mirage");
    assert_eq!(ts.scheme.family, Some("Ayu".to_string()));
    assert_eq!(ts.scheme.style, Some("Mirage".to_string()));

    Ok(())
}

#[test]
fn deserialize_ui_overrides() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_UI)?;

    assert_eq!(scheme.ui.background.normal.to_hex(), "111111");
    assert_eq!(scheme.ui.foreground.normal.to_hex(), "eeeeee");
    assert_eq!(scheme.ui.background.dark.to_hex(), "000000");

    Ok(())
}

#[test]
fn deserialize_ui_nested_overrides() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(
        r##"
scheme:
  system: "tinted8"
  author: "Test Author"
  name: "UI Test"
  system-version: "0.1.0"
  slug: "with-ui-nested"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
ui:
  background:
    normal: "#111111"
    dark:   "#000000"
  foreground:
    normal: "#eeeeee"
"##,
    )?;

    assert_eq!(scheme.ui.background.normal.to_hex(), "111111");
    assert_eq!(scheme.ui.foreground.normal.to_hex(), "eeeeee");
    assert_eq!(scheme.ui.background.dark.to_hex(), "000000");

    Ok(())
}

#[test]
fn deserialize_ui_normals_from_palette() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert_eq!(
        scheme.ui.background.normal.to_hex(),
        scheme.palette.black_normal.to_hex()
    );
    assert_eq!(
        scheme.ui.foreground.normal.to_hex(),
        scheme.palette.white_normal.to_hex()
    );
    assert_eq!(
        scheme.ui.search_text.to_hex(),
        scheme.palette.yellow_normal.to_hex()
    );

    Ok(())
}

#[test]
fn deserialize_syntax_overrides() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_SYNTAX)?;

    assert_eq!(scheme.syntax.comment.to_hex(), "888888");
    assert_eq!(scheme.syntax.string.default.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.string.quoted.to_hex(), "ddeeff");
    assert_eq!(scheme.syntax.constant.default.to_hex(), "112233");
    assert_eq!(scheme.syntax.keyword.default.to_hex(), "445566");

    Ok(())
}

#[test]
fn deserialize_syntax_inherits_from_parent() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_SYNTAX)?;

    assert_eq!(scheme.syntax.string.regexp.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.string.template.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.keyword.control.to_hex(), "445566");
    assert_eq!(scheme.syntax.keyword.declaration.to_hex(), "445566");

    Ok(())
}

#[test]
fn deserialize_syntax_normals_from_palette() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert_eq!(
        scheme.syntax.comment.to_hex(),
        scheme.palette.gray_dim.to_hex()
    );
    assert_eq!(
        scheme.syntax.string.default.to_hex(),
        scheme.palette.green_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.constant.default.to_hex(),
        scheme.palette.yellow_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.keyword.default.to_hex(),
        scheme.palette.magenta_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.markup.default.to_hex(),
        scheme.palette.cyan_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.diff.added.to_hex(),
        scheme.palette.green_bright.to_hex()
    );
    assert_eq!(
        scheme.syntax.diff.deleted.to_hex(),
        scheme.palette.red_bright.to_hex()
    );

    Ok(())
}

#[test]
fn deserialize_full_scheme() -> Result<(), TintedBuilderError> {
    let ts: Tinted8Scheme = serde_yaml::from_str(SCHEME_FULL)?;

    assert_eq!(ts.scheme.name, "Full Test Scheme");
    assert_eq!(ts.scheme.slug, "full-test");
    assert_eq!(ts.scheme.author, "Full Author <full@example.com>");
    assert_eq!(ts.scheme.theme_author, "Original Theme Author");
    assert_eq!(
        ts.scheme.description,
        Some("A complete test scheme".to_string())
    );
    assert_eq!(ts.scheme.supported_styling_version, "0.1.0".to_string());
    assert_eq!(ts.syntax.comment.to_hex(), "565f89");
    assert_eq!(ts.syntax.entity.name.default.to_hex(), "7aa2f7");
    assert_eq!(ts.syntax.entity.other.attribute_name.to_hex(), "e0af68");
    assert_eq!(ts.ui.background.normal.to_hex(), "ff0000");
    assert_eq!(ts.ui.selection.background.to_hex(), "33467c");

    Ok(())
}

#[test]
fn scheme_enum_wraps_tinted8() -> Result<(), TintedBuilderError> {
    let tinted8_scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;
    let scheme = Scheme::Tinted8(Box::new(tinted8_scheme));

    assert_eq!(scheme.get_scheme_name(), "Test Scheme");
    assert_eq!(scheme.get_scheme_slug(), "test-scheme");
    assert_eq!(scheme.get_scheme_author(), "Test Author <test@example.com>");

    Ok(())
}

#[test]
fn palette_has_all_variants() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert!(!scheme.palette.black_normal.to_hex().is_empty());
    assert!(!scheme.palette.black_bright.to_hex().is_empty());
    assert!(!scheme.palette.black_dim.to_hex().is_empty());
    assert!(!scheme.palette.gray_normal.to_hex().is_empty());
    assert!(!scheme.palette.orange_normal.to_hex().is_empty());
    assert!(!scheme.palette.brown_normal.to_hex().is_empty());

    Ok(())
}

const SCHEME_MINIMAL: &str = r##"
scheme:
  name: "Test Scheme"
  system: "tinted8"
  author: "Test Author <test@example.com>"
  system-version: "0.1.0"
  slug: "test-scheme"
variant: "dark"
palette:
  black:   "#131721"
  red:     "#f07178"
  green:   "#b8cc52"
  yellow:  "#ffb454"
  blue:    "#59c2ff"
  magenta: "#d2a6ff"
  cyan:    "#95e6cb"
  white:   "#e6e1cf"
name: "Test Scheme"
"##;

const SCHEME_WITH_FAMILY_STYLE: &str = r##"
scheme:
  system: "tinted8"
  author: "Test Author"
  system-version: "0.1.0"
variant: "light"
family: "Ayu"
style: "Mirage"
palette:
  black:   "#131721"
  red:     "#f07178"
  green:   "#b8cc52"
  yellow:  "#ffb454"
  blue:    "#59c2ff"
  magenta: "#d2a6ff"
  cyan:    "#95e6cb"
  white:   "#e6e1cf"
"##;

const SCHEME_WITH_UI: &str = r##"
scheme:
  system: "tinted8"
  author: "Test Author"
  name: "UI Test"
  system-version: "0.1.0"
  slug: "with-ui"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
ui:
  background.normal: "#111111"
  foreground.normal: "#eeeeee"
"##;

const SCHEME_WITH_SYNTAX: &str = r##"
scheme:
  system: "tinted8"
  author: "Test Author"
  slug: "with-syntax"
  system-version: 0.2.0
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
syntax:
  comment: "#888888"
  string: "#aabbcc"
  string.quoted: "#ddeeff"
  constant: "#112233"
  keyword: "#445566"
"##;

const SCHEME_FULL: &str = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  author: "Full Author <full@example.com>"
  theme-author: "Original Theme Author"
  name: "Full Test Scheme"
  slug: "full-test"
  description: "A complete test scheme"
variant: "dark"
palette:
  black:   "#1a1b26"
  red:     "#f7768e"
  green:   "#9ece6a"
  yellow:  "#e0af68"
  blue:    "#7aa2f7"
  magenta: "#bb9af7"
  cyan:    "#7dcfff"
  white:   "#c0caf5"
syntax:
  comment: "#565f89"
  string: "#9ece6a"
  constant: "#ff9e64"
  keyword: "#bb9af7"
  entity.name: "#7aa2f7"
  entity.other.attribute-name: "#e0af68"
  diff.added: "#9ece6a"
  diff.deleted: "#f7768e"
ui:
  background:
    normal: "#ff0000"
  foreground: "#c0caf5"
  selection.background: "#33467c"
"##;
