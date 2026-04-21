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

    assert_eq!(ts.scheme.name, "Ayu Mirage");
    assert_eq!(ts.scheme.slug, "ayu-mirage");
    assert_eq!(ts.scheme.family, Some("Ayu".to_string()));
    assert_eq!(ts.scheme.style, Some("Mirage".to_string()));

    Ok(())
}

#[test]
fn deserialize_ui_overrides() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_UI)?;

    assert_eq!(scheme.ui.global.background.normal.to_hex(), "111111");
    assert_eq!(scheme.ui.global.foreground.normal.to_hex(), "eeeeee");
    assert_eq!(scheme.ui.global.background.dark.to_hex(), "000000");

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
  supports:
    styling-spec: "0.2.0"
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
  global:
    background:
      normal: "#111111"
      dark:   "#000000"
    foreground:
      normal: "#eeeeee"
  cursor.normal.background: "#333333"
  cursor.normal.foreground: "#f1f1f1"
  cursor.muted.background: "#666666"
  accent.normal: "#aaaaaa"
  border.normal: "#bbbbbb"
  link.normal.background: "#cccccc"
  link.normal.foreground: "#dddddd"
"##,
    )?;

    assert_eq!(scheme.ui.global.background.normal.to_hex(), "111111");
    assert_eq!(scheme.ui.global.foreground.normal.to_hex(), "eeeeee");
    assert_eq!(scheme.ui.global.background.dark.to_hex(), "000000");
    assert_eq!(scheme.ui.cursor.normal.background.to_hex(), "333333");
    assert_eq!(scheme.ui.cursor.normal.foreground.to_hex(), "f1f1f1");
    assert_eq!(scheme.ui.cursor.muted.background.to_hex(), "666666");
    assert_eq!(scheme.ui.accent.normal.to_hex(), "aaaaaa");
    assert_eq!(scheme.ui.border.normal.to_hex(), "bbbbbb");
    assert_eq!(scheme.ui.link.normal.background.to_hex(), "cccccc");
    assert_eq!(scheme.ui.link.normal.foreground.to_hex(), "dddddd");

    Ok(())
}

#[test]
fn deserialize_ui_normals_from_palette() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert_eq!(
        scheme.ui.global.background.normal.to_hex(),
        scheme.palette.black_normal.to_hex()
    );
    assert_eq!(
        scheme.ui.global.foreground.normal.to_hex(),
        scheme.palette.white_normal.to_hex()
    );
    assert_eq!(
        scheme.ui.highlight.search.foreground.to_hex(),
        scheme.palette.yellow_normal.to_hex()
    );

    Ok(())
}

#[test]
fn deserialize_syntax_overrides() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_SYNTAX)?;

    assert_eq!(scheme.syntax.comment.default.to_hex(), "888888");
    assert_eq!(scheme.syntax.string.default.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.string.quoted.default.to_hex(), "ddeeff");
    assert_eq!(scheme.syntax.constant.default.to_hex(), "112233");
    assert_eq!(scheme.syntax.keyword.default.to_hex(), "445566");

    Ok(())
}

#[test]
fn deserialize_syntax_inherits_from_parent() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_SYNTAX)?;

    assert_eq!(scheme.syntax.string.default.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.string.template.to_hex(), "aabbcc");
    assert_eq!(scheme.syntax.keyword.control.default.to_hex(), "445566");
    assert_eq!(scheme.syntax.keyword.declaration.to_hex(), "445566");

    Ok(())
}

#[test]
fn deserialize_syntax_normals_from_palette() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    assert_eq!(
        scheme.syntax.comment.default.to_hex(),
        scheme.palette.gray_dim.to_hex()
    );
    assert_eq!(
        scheme.syntax.string.default.to_hex(),
        scheme.palette.green_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.constant.default.to_hex(),
        scheme.palette.orange_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.keyword.default.to_hex(),
        scheme.palette.magenta_normal.to_hex()
    );
    assert_eq!(
        scheme.syntax.markup.default.to_hex(),
        scheme.palette.orange_normal.to_hex()
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
    assert_eq!(ts.scheme.supports.styling_spec, "0.2.0".to_string());
    assert_eq!(ts.syntax.comment.default.to_hex(), "565f89");
    assert_eq!(ts.syntax.entity.name.default.to_hex(), "7aa2f7");
    assert_eq!(ts.syntax.entity.other.attribute_name.to_hex(), "e0af68");
    assert_eq!(ts.ui.global.background.normal.to_hex(), "ff0000");
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
  supports:
    styling-spec: "0.2.0"
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
  supports:
    styling-spec: "0.2.0"
  family: "Ayu"
  style: "Mirage"
variant: "light"
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
  supports:
    styling-spec: "0.2.0"
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
  global.background.normal: "#111111"
  global.foreground.normal: "#eeeeee"
  global.background.dark: "#000000"
"##;

const SCHEME_WITH_SYNTAX: &str = r##"
scheme:
  system: "tinted8"
  author: "Test Author"
  slug: "with-syntax"
  supports:
    styling-spec: 0.2.0
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
  supports:
    styling-spec: "0.2.0"
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
  entity: "#7aa2f7"
  entity.name: "#7aa2f7"
  entity.other.attribute-name: "#e0af68"
  markup.inserted: "#9ece6a"
  markup.deleted: "#f7768e"
ui:
  global:
    background:
      normal: "#ff0000"
    foreground.normal: "#c0caf5"
  selection.background: "#33467c"
"##;

#[test]
fn syntax_all_keys_accessible() -> Result<(), TintedBuilderError> {
    use tinted_builder::tinted8::SyntaxKey;

    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_MINIMAL)?;

    for key in SyntaxKey::variants() {
        let color = scheme.syntax.get_color(key);
        assert!(!color.to_hex().is_empty(), "Key {key} should have a color");
    }

    Ok(())
}

#[test]
fn syntax_new_scopes_parse_correctly() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_NEW_SCOPES)?;

    assert_eq!(scheme.syntax.comment.documentation.to_hex(), "aaaaaa");
    assert_eq!(scheme.syntax.string.other.to_hex(), "bbbbbb");
    assert_eq!(
        scheme.syntax.entity.name.function.constructor.to_hex(),
        "cccccc"
    );
    assert_eq!(scheme.syntax.entity.name.label.to_hex(), "dddddd");
    assert_eq!(scheme.syntax.keyword.control.import.to_hex(), "eeeeee");
    assert_eq!(scheme.syntax.keyword.control.flow.to_hex(), "111111");
    assert_eq!(scheme.syntax.support.function.builtin.to_hex(), "222222");
    assert_eq!(scheme.syntax.support.other.to_hex(), "333333");
    assert_eq!(scheme.syntax.variable.other.default.to_hex(), "444444");
    assert_eq!(scheme.syntax.variable.other.constant.to_hex(), "555555");
    assert_eq!(
        scheme.syntax.punctuation.definition.comment.to_hex(),
        "777777"
    );
    assert_eq!(scheme.syntax.markup.heading.to_hex(), "888888");
    assert_eq!(scheme.syntax.markup.list.numbered.to_hex(), "999999");
    assert_eq!(scheme.syntax.markup.list.unnumbered.to_hex(), "ababab");
    assert_eq!(scheme.syntax.markup.inserted.to_hex(), "bcbcbc");
    assert_eq!(scheme.syntax.markup.changed.to_hex(), "cdcdcd");
    assert_eq!(scheme.syntax.markup.deleted.to_hex(), "dedede");
    assert_eq!(scheme.syntax.source.to_hex(), "efefef");
    assert_eq!(scheme.syntax.text.to_hex(), "f0f0f0");
    assert_eq!(scheme.syntax.meta.default.to_hex(), "f1f1f1");

    Ok(())
}

#[test]
fn syntax_inheritance_direct_children() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_INHERITANCE)?;

    assert_eq!(scheme.syntax.keyword.default.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.control.default.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.declaration.to_hex(), "ff0000");

    assert_eq!(scheme.syntax.markup.default.to_hex(), "00ff00");
    assert_eq!(scheme.syntax.markup.list.default.to_hex(), "00ff00");
    assert_eq!(scheme.syntax.markup.inserted.to_hex(), "00ff00");

    Ok(())
}

#[test]
fn syntax_inheritance_grandchildren() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_GRANDCHILD_INHERITANCE)?;

    assert_eq!(scheme.syntax.keyword.default.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.control.default.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.control.import.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.control.flow.to_hex(), "ff0000");

    Ok(())
}

#[test]
fn syntax_partial_inheritance() -> Result<(), TintedBuilderError> {
    let scheme: Tinted8Scheme = serde_yaml::from_str(SCHEME_WITH_PARTIAL_INHERITANCE)?;

    assert_eq!(scheme.syntax.keyword.default.to_hex(), "ff0000");
    assert_eq!(scheme.syntax.keyword.control.default.to_hex(), "00ff00");
    assert_eq!(scheme.syntax.keyword.control.import.to_hex(), "00ff00");
    assert_eq!(scheme.syntax.keyword.control.flow.to_hex(), "0000ff");

    Ok(())
}

const SCHEME_WITH_NEW_SCOPES: &str = r##"
scheme:
  system: "tinted8"
  author: "Test"
  slug: "new-scopes"
  supports:
    styling-spec: "0.2.0"
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
  comment.documentation: "#aaaaaa"
  string.other: "#bbbbbb"
  entity.name.function.constructor: "#cccccc"
  entity.name.label: "#dddddd"
  keyword.control.import: "#eeeeee"
  keyword.control.flow: "#111111"
  support.function.builtin: "#222222"
  support.other: "#333333"
  variable.other: "#444444"
  variable.other.constant: "#555555"
  punctuation.definition.comment: "#777777"
  markup.heading: "#888888"
  markup.list.numbered: "#999999"
  markup.list.unnumbered: "#ababab"
  markup.inserted: "#bcbcbc"
  markup.changed: "#cdcdcd"
  markup.deleted: "#dedede"
  source: "#efefef"
  text: "#f0f0f0"
  meta: "#f1f1f1"
"##;

const SCHEME_WITH_INHERITANCE: &str = r##"
scheme:
  system: "tinted8"
  author: "Test"
  slug: "inheritance"
  supports:
    styling-spec: "0.2.0"
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
  keyword: "#ff0000"
  markup: "#00ff00"
"##;

const SCHEME_WITH_PARTIAL_INHERITANCE: &str = r##"
scheme:
  system: "tinted8"
  author: "Test"
  slug: "partial-inheritance"
  supports:
    styling-spec: "0.2.0"
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
  keyword: "#ff0000"
  keyword.control: "#00ff00"
  keyword.control.flow: "#0000ff"
"##;

const SCHEME_WITH_GRANDCHILD_INHERITANCE: &str = r##"
scheme:
  system: "tinted8"
  author: "Test"
  slug: "grandchild"
  supports:
    styling-spec: "0.2.0"
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
  keyword: "#ff0000"
"##;
