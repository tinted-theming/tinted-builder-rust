use core::iter::Iterator;
use serde::Deserialize;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct SyntaxSchema {
    syntax: Vec<SyntaxNode>,
}

#[derive(Debug, Deserialize)]
struct SyntaxNode {
    #[allow(dead_code)]
    key: String,
    scope: String,
    default: Option<String>,
    #[serde(default)]
    non_standard: bool,
    #[serde(default)]
    children: Vec<Self>,
}

#[derive(Debug)]
struct FlattenedNode {
    variant_name: String,
    scope: String,
    default_color: String,
    parent_scopes: Vec<String>,
    #[allow(dead_code)]
    non_standard: bool,
}

/// Generates Rust source from the tinted syntax schema at build time.
///
/// # Returns
/// - `()` after emitting Cargo rerun hints and writing generated code to `OUT_DIR`.
fn main() {
    println!("cargo:rerun-if-changed=src/scheme/tinted8/syntax_schema.yaml");

    let schema_path = Path::new("src/scheme/tinted8/syntax_schema.yaml");
    let schema_content =
        fs::read_to_string(schema_path).expect("Failed to read syntax_schema.yaml");
    let schema: SyntaxSchema =
        serde_yaml::from_str(&schema_content).expect("Failed to parse syntax_schema.yaml");
    let flattened = flatten_nodes(&schema.syntax, None, &[]);
    let generated = generate_code(&flattened, &schema);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("syntax_generated.rs");
    fs::write(&dest_path, generated).expect("Failed to write generated code");
}

/// Flattens the syntax tree into a linear list of nodes with inherited defaults.
///
/// # Arguments
/// - `nodes`: Current level of syntax nodes to flatten.
/// - `parent_default`: Default color inherited from the parent, if any.
/// - `parent_scopes`: Ancestor scopes used to record parent relationships.
///
/// # Returns
/// - A flat list of `FlattenedNode` entries that includes inherited defaults and parent scopes.
fn flatten_nodes(
    nodes: &[SyntaxNode],
    parent_default: Option<&str>,
    parent_scopes: &[String],
) -> Vec<FlattenedNode> {
    let mut result = Vec::new();

    for node in nodes {
        let variant_name = scope_to_variant_name(&node.scope);
        let default_color = node
            .default
            .as_deref()
            .or(parent_default)
            .unwrap_or_else(|| panic!("default type is not set for {variant_name}"))
            .to_string();

        result.push(FlattenedNode {
            variant_name,
            scope: node.scope.clone(),
            default_color: default_color.clone(),
            parent_scopes: parent_scopes.to_vec(),
            non_standard: node.non_standard,
        });

        if !node.children.is_empty() {
            let mut new_parent_scopes = parent_scopes.to_vec();
            new_parent_scopes.push(node.scope.clone());
            let children = flatten_nodes(
                &node.children,
                Some(&default_color),
                new_parent_scopes.as_ref(),
            );
            result.extend(children);
        }
    }

    result
}

/// Converts a dotted scope into a ``PascalCase`` enum variant name.
///
/// # Arguments
/// - `scope`: Dotted scope string (e.g., `comment.line`).
///
/// # Returns
/// - A ``PascalCase`` variant name derived from the scope.
fn scope_to_variant_name(scope: &str) -> String {
    scope
        .split('.')
        .map(|part| part.split('-').map(capitalize).collect::<String>())
        .collect()
}

/// Capitalizes the first character in a segment, leaving the rest unchanged.
///
/// # Arguments
/// - `s`: Segment to capitalize.
///
/// # Returns
/// - The segment with its first character uppercased.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |c| {
        c.to_uppercase().collect::<String>() + chars.as_str()
    })
}

/// Produces all generated Rust code for syntax keys and helpers.
///
/// # Arguments
/// - `nodes`: Flattened syntax nodes used for code emission.
/// - `schema`: Full syntax schema used for nested construction.
///
/// # Returns
/// - A single `String` containing all generated Rust code.
fn generate_code(nodes: &[FlattenedNode], schema: &SyntaxSchema) -> String {
    let mut code = String::new();

    generate_syntax_key_enum(&mut code, nodes);
    code.push_str("\n\n");
    generate_basic_syntax_struct(&mut code, nodes);
    code.push_str("\n\n");
    generate_valid_syntax_keys(&mut code, nodes);
    code.push_str("\n\n");
    generate_get_palette_color(&mut code);
    code.push_str("\n\n");
    generate_try_from_basic(&mut code, &schema.syntax);

    code
}

/// Emits the `SyntaxKey` enum and associated helper methods.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
/// - `nodes`: Flattened syntax nodes to define enum variants and helpers.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_syntax_key_enum(code: &mut String, nodes: &[FlattenedNode]) {
    code.push_str("#[non_exhaustive]\n");
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub enum SyntaxKey {\n");
    for node in nodes {
        let _ = writeln!(code, "    {},", node.variant_name);
    }
    code.push_str("}\n\n");

    code.push_str("#[allow(clippy::match_same_arms)]\n");
    code.push_str("impl SyntaxKey {\n");
    code.push_str("    #[must_use]\n");
    code.push_str("    #[allow(clippy::too_many_lines)]\n");
    code.push_str("    pub const fn as_str(&self) -> &str {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        let _ = writeln!(
            code,
            "            Self::{} => \"{}\",",
            node.variant_name, node.scope
        );
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    code.push_str("    #[must_use]\n");
    code.push_str("    #[allow(clippy::too_many_lines)]\n");
    code.push_str("    pub const fn variants() -> &'static [Self] {\n");
    code.push_str("        &[\n");
    for node in nodes {
        let _ = writeln!(code, "            Self::{},", node.variant_name);
    }
    code.push_str("        ]\n");
    code.push_str("    }\n\n");

    code.push_str("    #[must_use]\n");
    code.push_str("    #[allow(clippy::too_many_lines)]\n");
    code.push_str("    pub const fn default_color(&self) -> &str {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        let _ = writeln!(
            code,
            "            Self::{} => \"{}\",",
            node.variant_name, node.default_color
        );
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    code.push_str("    #[must_use]\n");
    code.push_str("    #[allow(clippy::too_many_lines)]\n");
    code.push_str("    pub const fn parent_scopes(&self) -> &[&str] {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        if node.parent_scopes.is_empty() {
            let _ = writeln!(code, "            Self::{} => &[],", node.variant_name);
        } else {
            let parents: Vec<String> = node
                .parent_scopes
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect();
            let _ = writeln!(
                code,
                "            Self::{} => &[{}],",
                node.variant_name,
                parents.join(", ")
            );
        }
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push('\n');
    code.push_str("    #[must_use]\n");
    code.push_str("    #[allow(clippy::too_many_lines)]\n");
    code.push_str("    pub const fn is_non_standard(&self) -> bool {\n");
    code.push_str("        match self {\n");
    for node in nodes {
        let _ = writeln!(
            code,
            "            Self::{} => {},",
            node.variant_name, node.non_standard
        );
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("impl std::fmt::Display for SyntaxKey {\n");
    code.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    code.push_str("        write!(f, \"{}\", self.as_str())\n");
    code.push_str("    }\n");
    code.push_str("}\n");
}

/// Emits the `BasicSyntax` struct and its accessor.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
/// - `nodes`: Flattened syntax nodes to define struct fields and accessors.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_basic_syntax_struct(code: &mut String, nodes: &[FlattenedNode]) {
    code.push_str("#[derive(serde::Deserialize, serde::Serialize, Default)]\n");
    code.push_str("#[serde(deny_unknown_fields)]\n");
    code.push_str("pub struct BasicSyntax {\n");

    for node in nodes {
        let rust_field = node.scope.replace(['.', '-'], "_");
        if rust_field != node.scope {
            let _ = writeln!(code, "    #[serde(rename = \"{}\")]", node.scope);
        }
        let _ = writeln!(code, "    pub {rust_field}: Option<String>,");
    }

    code.push_str("}\n\n");

    code.push_str("impl BasicSyntax {\n");
    code.push_str("    #[allow(clippy::too_many_lines)]");
    code.push_str("    pub fn get_field(&self, key: &SyntaxKey) -> Option<&str> {\n");
    code.push_str("        match key {\n");
    for node in nodes {
        let rust_field = node.scope.replace(['.', '-'], "_");
        let _ = writeln!(
            code,
            "            SyntaxKey::{} => self.{rust_field}.as_deref(),",
            node.variant_name
        );
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n");
}

/// Emits the static list of valid syntax scopes.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
/// - `nodes`: Flattened syntax nodes to list valid scopes.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_valid_syntax_keys(code: &mut String, nodes: &[FlattenedNode]) {
    code.push_str("#[allow(dead_code)]\npub const VALID_SYNTAX_SCOPES: &[&str] = &[\n");
    for node in nodes {
        let _ = writeln!(code, "    \"{}\",", node.scope);
    }
    code.push_str("];\n");
}

/// Emits a helper that maps color names to palette fields.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_get_palette_color(code: &mut String) {
    code.push_str("fn get_palette_color(palette: &Palette, color_type: &ColorType) -> Result<Color, TintedBuilderError> {\n");
    code.push_str("    #[allow(clippy::match_same_arms)]\n");
    code.push_str("    match color_type {\n");

    let colors = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white", "orange", "gray",
        "brown",
    ];
    let variants = ["normal", "bright", "dim"];

    for color in &colors {
        for variant in &variants {
            let color_name: String = color
                .chars()
                .enumerate()
                .map(|(index, char)| {
                    if index == 0 {
                        char.to_uppercase().collect::<String>()
                    } else {
                        char.to_string()
                    }
                })
                .collect();
            let variant_name: String = variant
                .chars()
                .enumerate()
                .map(|(index, char)| {
                    if index == 0 {
                        char.to_uppercase().collect::<String>()
                    } else {
                        char.to_string()
                    }
                })
                .collect();
            let _ = writeln!(
                code,
                r"        ColorType(ColorName::{color_name}, ColorVariant::{variant_name}) => Color::new(&palette.{color}_{variant}.to_hex(), Some(ColorName::{color_name}), Some(ColorVariant::{variant_name})),"
            );
        }
    }
    code.push_str("        _ => Ok(palette.white_normal.clone()),\n");
    code.push_str("    }\n");
    code.push_str("}\n");
}

/// Emits the conversion function from `BasicSyntax` to `Syntax`.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
/// - `nodes`: Root syntax nodes to drive nested construction.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_try_from_basic(code: &mut String, nodes: &[SyntaxNode]) {
    code.push_str("#[allow(clippy::too_many_lines)]\n");
    code.push_str(
        "pub fn generated_try_from_basic(basic: &BasicSyntax, palette: &Palette) -> Result<Syntax, SyntaxError> {\n",
    );

    generate_syntax_construction(code, nodes, "    ", None);

    code.push_str("    #[allow(clippy::redundant_field_names)]");
    code.push_str("    Ok(Syntax {\n");
    for node in nodes {
        let var_name = scope_to_var_name(&node.scope);
        let field_name = scope_to_field_name(&node.scope);
        let _ = writeln!(code, "        {field_name}: {var_name},");
    }
    code.push_str("    })\n");
    code.push_str("}\n");
}

/// Emits the nested syntax construction code for each schema node.
///
/// # Arguments
/// - `code`: Buffer to append generated code to.
/// - `nodes`: Current schema nodes to emit construction for.
/// - `indent`: Indentation prefix for emitted lines.
///
/// # Returns
/// - `()` after appending to `code`.
fn generate_syntax_construction(
    code: &mut String,
    nodes: &[SyntaxNode],
    indent: &str,
    parent_default: Option<&str>,
) {
    for node in nodes {
        let variant_name = scope_to_var_name(&node.scope);
        let basic_field = node.scope.replace(['.', '-'], "_");
        let default_color = node
            .default
            .as_deref()
            .or(parent_default)
            .unwrap_or_else(|| panic!("default type is not set for {variant_name}"));
        let parent_chain = build_parent_chain(&node.scope);

        if node.children.is_empty() {
            let _ = writeln!(
                code,
                "{indent}let {variant_name} = parse_or_inherit(&[{}], &get_palette_color(palette, &ColorType::from_str(\"{default_color}\")?)?)?;",
                format_parent_chain(&basic_field, &parent_chain)
            );
        } else {
            generate_syntax_construction(code, &node.children, indent, Some(default_color));

            let struct_name = scope_to_struct_name(&node.scope);
            let _ = writeln!(code, "{indent}let {variant_name} = {struct_name} {{");
            let _ = writeln!(
                code,
                "{indent}    default: parse_or_inherit(&[{}], &get_palette_color(palette, &ColorType::from_str(\"{default_color}\")?)?)?,",
                format_parent_chain(&basic_field, &parent_chain)
            );

            for child in &node.children {
                let child_var = scope_to_var_name(&child.scope);
                let child_field = scope_to_field_name(&child.scope);
                let _ = writeln!(code, "{indent}    {child_field}: {child_var},");
            }

            let _ = writeln!(code, "{indent}}};");
        }
    }
}

/// Converts a scope into a safe local variable name.
///
/// # Arguments
/// - `scope`: Dotted scope string.
///
/// # Returns
/// - A Rust-safe local variable name (with keyword escapes as needed).
fn scope_to_var_name(scope: &str) -> String {
    let var = scope.replace(['.', '-'], "_");
    let var_str = var.as_str();

    match var_str {
        "class" => "r#class".to_string(),
        "enum" => "r#enum".to_string(),
        "struct" => "r#struct".to_string(),
        "type" => "r#type".to_string(),
        _ => var,
    }
}

/// Builds a list of parent scope field names from most specific to root.
///
/// # Arguments
/// - `scope`: Dotted scope string.
///
/// # Returns
/// - A vector of parent field names, ordered from immediate parent to root.
fn build_parent_chain(scope: &str) -> Vec<String> {
    let parts: Vec<&str> = scope.split('.').collect();
    let mut chain = Vec::new();

    for i in (1..parts.len()).rev() {
        let parent_scope = parts[..i].join(".");
        chain.push(parent_scope.replace(['.', '-'], "_"));
    }

    chain
}

/// Formats a parent chain into `basic.<field>.as_deref()` expressions.
///
/// # Arguments
/// - `field`: The current field name to start the chain.
/// - `parents`: Parent field names to append in order.
///
/// # Returns
/// - A comma-separated list of `basic.<field>.as_deref()` expressions.
fn format_parent_chain(field: &str, parents: &[String]) -> String {
    let mut parts = vec![format!("basic.{field}.as_deref()")];
    for parent in parents {
        parts.push(format!("basic.{parent}.as_deref()"));
    }
    parts.join(", ")
}

/// Converts a scope into a struct field name, preserving Rust keywords.
///
/// # Arguments
/// - `scope`: Dotted scope string.
///
/// # Returns
/// - A Rust field name derived from the last scope segment.
fn scope_to_field_name(scope: &str) -> String {
    let field = scope.split('.').next_back().unwrap_or(scope);
    let rust_field = field.replace('-', "_");
    let rust_field_str = rust_field.as_ref();

    match rust_field_str {
        "class" => "r#class".to_string(),
        "enum" => "r#enum".to_string(),
        "struct" => "r#struct".to_string(),
        "type" => "r#type".to_string(),
        _ => rust_field,
    }
}

/// Converts a scope into the nested `Syntax*` struct name.
///
/// # Arguments
/// - `scope`: Dotted scope string.
///
/// # Returns
/// - A ``PascalCase`` struct name prefixed with `Syntax`.
fn scope_to_struct_name(scope: &str) -> String {
    let mut name = String::from("Syntax");

    for part in scope.split('.') {
        for segment in part.split('-') {
            name.push_str(&capitalize(segment));
        }
    }
    name
}
