use crate::bindings::exports::tinted_theming::tinted_builder::{
    colors::Guest as ColorsGuest,
    renderer::Guest as RendererGuest,
    schemes::{Guest as SchemesGuest, GuestScheme, SchemeBorrow},
    tinted8::{Guest as Tinted8Guest, GuestTinted8Scheme, Tinted8Scheme as WitTinted8Scheme},
    types::{
        Color as WitColor, ColorName as WitColorName, ColorVariant as WitColorVariant,
        PaletteEntry, SchemeMetadata, SchemeSystem as WitSchemeSystem,
        SchemeVariant as WitSchemeVariant, Tinted8SchemeMetadata,
    },
};

use crate::scheme::tinted8::structure::ui::UiKey;
use crate::{Color, ColorName, ColorVariant, Scheme, SchemeSystem, SchemeVariant, Template};

// ---------------------------------------------------------------------------
// Color conversions
// ---------------------------------------------------------------------------

impl From<&Color> for WitColor {
    fn from(c: &Color) -> Self {
        Self {
            hex: c.to_hex(),
            hex_r: c.hex.0.clone(),
            hex_g: c.hex.1.clone(),
            hex_b: c.hex.2.clone(),
            hex_bgr: format!("{}{}{}", c.hex.2, c.hex.1, c.hex.0),
            rgb_r: c.rgb.0,
            rgb_g: c.rgb.1,
            rgb_b: c.rgb.2,
            rgb16_r: u16::from(c.rgb.0) * 257,
            rgb16_g: u16::from(c.rgb.1) * 257,
            rgb16_b: u16::from(c.rgb.2) * 257,
            dec_r: c.dec.0,
            dec_g: c.dec.1,
            dec_b: c.dec.2,
            name: (&c.name).into(),
            variant: (&c.variant).into(),
        }
    }
}

impl TryFrom<&WitColor> for Color {
    type Error = String;

    fn try_from(c: &WitColor) -> Result<Self, Self::Error> {
        Color::new(&c.hex, Some(c.name.into()), Some(c.variant.into())).map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Palette helpers
// ---------------------------------------------------------------------------

fn palette_to_entries(palette: &std::collections::HashMap<String, Color>) -> Vec<PaletteEntry> {
    let mut entries: Vec<PaletteEntry> = palette
        .iter()
        .map(|(key, color)| PaletteEntry {
            key: key.clone(),
            color: color.into(),
        })
        .collect();
    entries.sort_by(|a, b| a.key.cmp(&b.key));
    entries
}

fn scheme_to_palette_entries(scheme: &Scheme) -> Vec<PaletteEntry> {
    match scheme {
        Scheme::Base16(s) => palette_to_entries(&s.palette),
        Scheme::Base24(s) => palette_to_entries(&s.palette),
        Scheme::Tinted8(s) => tinted8_palette_entries(&s.palette),
    }
}

fn tinted8_palette_entries(
    palette: &crate::scheme::tinted8::structure::Palette,
) -> Vec<PaletteEntry> {
    crate::scheme::tinted8::structure::Palette::get_color_list()
        .into_iter()
        .filter_map(|(name, variant)| {
            palette
                .get_color(&name, &variant)
                .map(|color| PaletteEntry {
                    key: format!("{name}-{variant}"),
                    color: color.into(),
                })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Component struct
// ---------------------------------------------------------------------------

pub struct Component;

// ---------------------------------------------------------------------------
// colors interface
// ---------------------------------------------------------------------------

impl ColorsGuest for Component {
    fn create(
        hex: String,
        name: Option<WitColorName>,
        variant: Option<WitColorVariant>,
    ) -> Result<WitColor, String> {
        let color = Color::new(&hex, name.map(Into::into), variant.map(Into::into))
            .map_err(|e| e.to_string())?;
        Ok((&color).into())
    }

    fn to_variant(color: WitColor, target_variant: WitColorVariant) -> Result<WitColor, String> {
        let rust_color = Color::try_from(&color)?;
        let derived = rust_color
            .try_to_variant(&target_variant.into())
            .map_err(|e| e.to_string())?;
        Ok((&derived).into())
    }

    fn to_color(color: WitColor, target_name: WitColorName) -> Result<WitColor, String> {
        let rust_color = Color::try_from(&color)?;
        let derived = rust_color
            .try_to_color(&target_name.into())
            .map_err(|e| e.to_string())?;
        Ok((&derived).into())
    }
}

// ---------------------------------------------------------------------------
// schemes interface
// ---------------------------------------------------------------------------

pub struct SchemeResource {
    inner: Scheme,
}

impl SchemesGuest for Component {
    type Scheme = SchemeResource;
}

impl GuestScheme for SchemeResource {
    fn parse(
        yaml: String,
    ) -> Result<crate::bindings::exports::tinted_theming::tinted_builder::schemes::Scheme, String>
    {
        let scheme = Scheme::from_yaml(&yaml).map_err(|e| e.to_string())?;
        Ok(
            crate::bindings::exports::tinted_theming::tinted_builder::schemes::Scheme::new(
                SchemeResource { inner: scheme },
            ),
        )
    }

    fn metadata(&self) -> SchemeMetadata {
        let desc = self.inner.get_scheme_description();
        SchemeMetadata {
            system: (&self.inner.get_scheme_system()).into(),
            name: self.inner.get_scheme_name(),
            slug: self.inner.get_scheme_slug(),
            author: self.inner.get_scheme_author(),
            description: if desc.is_empty() { None } else { Some(desc) },
            variant: (&self.inner.get_scheme_variant()).into(),
        }
    }

    fn palette(&self) -> Vec<PaletteEntry> {
        scheme_to_palette_entries(&self.inner)
    }
}

// ---------------------------------------------------------------------------
// renderer interface
// ---------------------------------------------------------------------------

impl RendererGuest for Component {
    fn render(scheme: SchemeBorrow<'_>, template_content: String) -> Result<String, String> {
        let scheme_res = scheme.get::<SchemeResource>();
        let template = Template::new(template_content, scheme_res.inner.clone());
        template.render().map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// tinted8 interface
// ---------------------------------------------------------------------------

pub struct Tinted8SchemeResource {
    inner: crate::tinted8::Scheme,
}

impl Tinted8Guest for Component {
    type Tinted8Scheme = Tinted8SchemeResource;

    fn supported_builder_spec_version() -> String {
        crate::tinted8::SUPPORTED_BUILDER_SPEC_VERSION.to_string()
    }

    fn supported_styling_spec_version() -> String {
        crate::tinted8::SUPPORTED_STYLING_SPEC_VERSION.to_string()
    }
}

impl GuestTinted8Scheme for Tinted8SchemeResource {
    fn parse(yaml: String) -> Result<WitTinted8Scheme, String> {
        let scheme: crate::tinted8::Scheme =
            serde_yaml::from_str(&yaml).map_err(|e| e.to_string())?;
        Ok(WitTinted8Scheme::new(Tinted8SchemeResource {
            inner: scheme,
        }))
    }

    fn metadata(&self) -> Tinted8SchemeMetadata {
        let meta = &self.inner.scheme;
        Tinted8SchemeMetadata {
            system: (&meta.system).into(),
            name: meta.name.clone(),
            slug: meta.slug.clone(),
            author: meta.author.clone(),
            theme_author: meta.theme_author.clone(),
            description: meta.description.clone(),
            variant: (&self.inner.variant).into(),
            supports_styling_spec: meta.supports.styling_spec.clone(),
            family: meta.family.clone(),
            style: meta.style.clone(),
        }
    }

    fn palette(&self) -> Vec<PaletteEntry> {
        tinted8_palette_entries(&self.inner.palette)
    }

    fn syntax_color(&self, key: String) -> Result<WitColor, String> {
        crate::tinted8::SyntaxKey::variants()
            .iter()
            .find(|k| k.as_str() == key)
            .map(|k| self.inner.syntax.get_color(k).into())
            .ok_or_else(|| format!("invalid syntax key: {key}"))
    }

    fn ui_color(&self, key: String) -> Result<WitColor, String> {
        UiKey::variants()
            .iter()
            .find(|k| k.to_string() == key)
            .map(|k| self.inner.ui.get_color(k).into())
            .ok_or_else(|| format!("invalid UI key: {key}"))
    }

    fn syntax_keys() -> Vec<String> {
        crate::tinted8::SyntaxKey::variants()
            .iter()
            .map(|k| k.as_str().to_string())
            .collect()
    }

    fn ui_keys() -> Vec<String> {
        UiKey::variants().iter().map(|k| k.to_string()).collect()
    }

    fn render(&self, template_content: String) -> Result<String, String> {
        let scheme = Scheme::Tinted8(Box::new(self.inner.clone()));
        let template = Template::new(template_content, scheme);
        template.render().map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Enum conversion macro — generates From impls in both directions
// ---------------------------------------------------------------------------

macro_rules! enum_conversions {
    ($rust_ty:ty, $wit_ty:ty, $($rust_variant:ident => $wit_variant:ident),+ $(,)?) => {
        impl From<&$rust_ty> for $wit_ty {
            fn from(v: &$rust_ty) -> Self {
                match v {
                    $(<$rust_ty>::$rust_variant => <$wit_ty>::$wit_variant),+
                }
            }
        }

        impl From<$wit_ty> for $rust_ty {
            fn from(v: $wit_ty) -> Self {
                match v {
                    $(<$wit_ty>::$wit_variant => <$rust_ty>::$rust_variant),+
                }
            }
        }
    };
}

enum_conversions!(ColorName, WitColorName,
    Black => Black, Red => Red, Green => Green, Yellow => Yellow,
    Blue => Blue, Magenta => Magenta, Cyan => Cyan, White => White,
    Orange => Orange, Gray => Gray, Brown => Brown, Other => Other,
);

enum_conversions!(ColorVariant, WitColorVariant,
    Dim => Dim, Normal => Normal, Bright => Bright,
);

enum_conversions!(SchemeSystem, WitSchemeSystem,
    Base16 => Base16, Base24 => Base24, Tinted8 => Tinted8,
);

enum_conversions!(SchemeVariant, WitSchemeVariant,
    Dark => Dark, Light => Light,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::exports::tinted_theming::tinted_builder::{
        colors::Guest as ColorsGuest,
        schemes::GuestScheme,
        tinted8::{Guest as Tinted8Guest, GuestTinted8Scheme},
    };

    const SCHEME_BASE16_YAML: &str = r##"
system: "base16"
name: "Silk Light"
slug: "silk-light"
author: "Gabriel Fontes"
variant: "light"
palette:
  base00: "#E9F1EF"
  base01: "#CCD4D3"
  base02: "#90B7B6"
  base03: "#5C787B"
  base04: "#4B5B5F"
  base05: "#385156"
  base06: "#0e3c46"
  base07: "#D2FAFF"
  base08: "#CF432E"
  base09: "#D27F46"
  base0A: "#CFAD25"
  base0B: "#6CA38C"
  base0C: "#329CA2"
  base0D: "#39AAC9"
  base0E: "#6E6582"
  base0F: "#865369"
"##;

    const SCHEME_TINTED8_YAML: &str = r##"
scheme:
  system: "tinted8"
  supports:
    styling-spec: "0.2.0"
  name: "Catppuccin Mocha"
  author: "https://github.com/catppuccin/catppuccin"
variant: "dark"
palette:
  black: "#1e1e2e"
  white: "#cdd6f4"
  red: "#f38ba8"
  yellow: "#f9e2af"
  green: "#a6e3a1"
  cyan: "#94e2d5"
  blue: "#89b4fa"
  magenta: "#cba6f7"
"##;

    // -----------------------------------------------------------------------
    // Color conversions
    // -----------------------------------------------------------------------

    #[test]
    fn color_to_wit_color() {
        let color = Color::new(
            "ff8800",
            Some(ColorName::Orange),
            Some(ColorVariant::Normal),
        )
        .expect("valid color");
        let wit: WitColor = (&color).into();

        assert_eq!(wit.hex, "ff8800");
        assert_eq!(wit.hex_r, "ff");
        assert_eq!(wit.hex_g, "88");
        assert_eq!(wit.hex_b, "00");
        assert_eq!(wit.hex_bgr, "0088ff");
        assert_eq!(wit.rgb_r, 255);
        assert_eq!(wit.rgb_g, 136);
        assert_eq!(wit.rgb_b, 0);
        assert_eq!(wit.rgb16_r, 255 * 257);
        assert_eq!(wit.rgb16_g, 136 * 257);
        assert_eq!(wit.rgb16_b, 0);
    }

    #[test]
    fn wit_color_roundtrip() {
        let original = Color::new("aabbcc", Some(ColorName::Blue), Some(ColorVariant::Dim))
            .expect("valid color");
        let wit: WitColor = (&original).into();
        let back = Color::try_from(&wit).expect("roundtrip");

        assert_eq!(original.to_hex(), back.to_hex());
        assert_eq!(original.rgb, back.rgb);
    }

    // -----------------------------------------------------------------------
    // Enum conversions
    // -----------------------------------------------------------------------

    #[test]
    fn color_name_roundtrip() {
        for name in ColorName::get_list() {
            let wit: WitColorName = name.into();
            let back: ColorName = wit.into();
            assert_eq!(format!("{name:?}"), format!("{back:?}"));
        }
    }

    #[test]
    fn color_variant_roundtrip() {
        for variant in ColorVariant::get_list() {
            let wit: WitColorVariant = variant.into();
            let back: ColorVariant = wit.into();
            assert_eq!(format!("{variant:?}"), format!("{back:?}"));
        }
    }

    #[test]
    fn scheme_system_roundtrip() {
        for system in SchemeSystem::variants() {
            let wit: WitSchemeSystem = system.into();
            let back: SchemeSystem = wit.into();
            assert_eq!(system, &back);
        }
    }

    #[test]
    fn scheme_variant_roundtrip() {
        let dark_wit: WitSchemeVariant = (&SchemeVariant::Dark).into();
        let light_wit: WitSchemeVariant = (&SchemeVariant::Light).into();
        assert_eq!(SchemeVariant::Dark, SchemeVariant::from(dark_wit));
        assert_eq!(SchemeVariant::Light, SchemeVariant::from(light_wit));
    }

    // -----------------------------------------------------------------------
    // colors interface
    // -----------------------------------------------------------------------

    #[test]
    fn colors_create() {
        let wit = Component::create("ff0000".to_string(), None, None).expect("create color");
        assert_eq!(wit.hex, "ff0000");
        assert_eq!(wit.rgb_r, 255);
        assert_eq!(wit.rgb_g, 0);
        assert_eq!(wit.rgb_b, 0);
    }

    #[test]
    fn colors_create_invalid_hex() {
        let result = Component::create("zzzzzz".to_string(), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn colors_to_variant() {
        let normal = Component::create(
            "ff0000".to_string(),
            Some(WitColorName::Red),
            Some(WitColorVariant::Normal),
        )
        .expect("create color");
        let dim = Component::to_variant(normal, WitColorVariant::Dim).expect("to dim");
        assert_eq!(dim.name, WitColorName::Red);
        assert_eq!(dim.variant, WitColorVariant::Dim);
    }

    #[test]
    fn colors_to_color() {
        let yellow = Component::create(
            "ffff00".to_string(),
            Some(WitColorName::Yellow),
            Some(WitColorVariant::Normal),
        )
        .expect("create yellow");
        let orange = Component::to_color(yellow, WitColorName::Orange).expect("to orange");
        assert_eq!(orange.name, WitColorName::Orange);
    }

    // -----------------------------------------------------------------------
    // schemes interface (construct SchemeResource directly)
    // -----------------------------------------------------------------------

    fn make_base16_resource() -> SchemeResource {
        let scheme = Scheme::from_yaml(SCHEME_BASE16_YAML).expect("parse base16");
        SchemeResource { inner: scheme }
    }

    #[test]
    fn scheme_metadata_base16() {
        let resource = make_base16_resource();
        let meta = resource.metadata();
        assert_eq!(meta.name, "Silk Light");
        assert_eq!(meta.system, WitSchemeSystem::Base16);
        assert_eq!(meta.variant, WitSchemeVariant::Light);
    }

    #[test]
    fn scheme_palette_base16() {
        let resource = make_base16_resource();
        let entries = resource.palette();
        assert_eq!(entries.len(), 16);
        let base0a = entries.iter().find(|e| e.key == "base0A").expect("base0A");
        assert_eq!(base0a.color.hex, "cfad25");
    }

    #[test]
    fn scheme_parse_invalid_yaml() {
        let result = Scheme::from_yaml("not: valid: yaml: [");
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // renderer (test via Template directly, same path as RendererGuest)
    // -----------------------------------------------------------------------

    #[test]
    fn renderer_render_base16() {
        let resource = make_base16_resource();
        let template = Template::new("bg: #{{base00-hex}}".to_string(), resource.inner);
        let output = template.render().expect("render");
        assert_eq!(output, "bg: #e9f1ef");
    }

    // -----------------------------------------------------------------------
    // tinted8 interface (construct Tinted8SchemeResource directly)
    // -----------------------------------------------------------------------

    fn make_tinted8_resource() -> Tinted8SchemeResource {
        let scheme: crate::tinted8::Scheme =
            serde_yaml::from_str(SCHEME_TINTED8_YAML).expect("parse tinted8");
        Tinted8SchemeResource { inner: scheme }
    }

    #[test]
    fn tinted8_metadata() {
        let resource = make_tinted8_resource();
        let meta = resource.metadata();
        assert_eq!(meta.name, "Catppuccin Mocha");
        assert_eq!(meta.system, WitSchemeSystem::Tinted8);
        assert_eq!(meta.variant, WitSchemeVariant::Dark);
    }

    #[test]
    fn tinted8_palette() {
        let resource = make_tinted8_resource();
        let entries = resource.palette();
        // 11 colors x 3 variants = 33
        assert_eq!(entries.len(), 33);
        let red_normal = entries
            .iter()
            .find(|e| e.key == "red-normal")
            .expect("red-normal");
        assert_eq!(red_normal.color.hex, "f38ba8");
    }

    #[test]
    fn tinted8_syntax_color() {
        let resource = make_tinted8_resource();
        let color = resource
            .syntax_color("comment".to_string())
            .expect("comment");
        assert!(!color.hex.is_empty());
    }

    #[test]
    fn tinted8_syntax_color_invalid() {
        let resource = make_tinted8_resource();
        let result = resource.syntax_color("nonexistent.scope".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn tinted8_ui_color() {
        let resource = make_tinted8_resource();
        let color = resource
            .ui_color("global.background.normal".to_string())
            .expect("global.background.normal");
        assert!(!color.hex.is_empty());
    }

    #[test]
    fn tinted8_ui_color_invalid() {
        let resource = make_tinted8_resource();
        let result = resource.ui_color("nonexistent.key".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn tinted8_syntax_keys_not_empty() {
        let keys = Tinted8SchemeResource::syntax_keys();
        assert!(!keys.is_empty());
        assert!(keys.contains(&"comment".to_string()));
        assert!(keys.contains(&"keyword.control.import".to_string()));
    }

    #[test]
    fn tinted8_ui_keys_not_empty() {
        let keys = Tinted8SchemeResource::ui_keys();
        assert!(!keys.is_empty());
        assert!(keys.contains(&"global.background.normal".to_string()));
    }

    #[test]
    fn tinted8_render() {
        let resource = make_tinted8_resource();
        let output = resource
            .render("{{scheme.name}}".to_string())
            .expect("render");
        assert_eq!(output, "Catppuccin Mocha");
    }

    #[test]
    fn tinted8_version_constants() {
        let builder = Component::supported_builder_spec_version();
        let styling = Component::supported_styling_spec_version();
        assert!(!builder.is_empty());
        assert!(!styling.is_empty());
    }
}
