use serde::Serialize;

use crate::{
    scheme::tinted8::{yaml::BasicUi, Palette},
    utils::parse_or_inherit,
    Color, SchemeVariant, TintedBuilderError,
};
use std::fmt;

macro_rules! define_ui_keys {
    ($($variant:ident => $str:literal),* $(,)?) => {
        #[non_exhaustive]
        #[derive(Debug, Clone)]
        pub enum UiKey {
            $($variant),*
        }

        impl UiKey {
            const fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $str),*
                }
            }

            #[must_use]
            pub const fn variants() -> &'static [Self] {
                &[$(Self::$variant),*]
            }
        }
    };
}

define_ui_keys! {
    GlobalBackgroundNormal => "global.background.normal",
    GlobalBackgroundDark => "global.background.dark",
    GlobalBackgroundLight => "global.background.light",
    Deprecated => "deprecated",
    Accent => "accent",
    Border => "border",
    CursorNormal => "cursor.normal",
    CursorMuted => "cursor.muted",
    GlobalForegroundNormal => "global.foreground.normal",
    GlobalForegroundDark => "global.foreground.dark",
    GlobalForegroundLight => "global.foreground.light",
    GutterBackground => "gutter.background",
    GutterForeground => "gutter.foreground",
    HighlightTextBackground => "highlight.text.background",
    HighlightTextForeground => "highlight.text.foreground",
    HighlightTextActiveBackground => "highlight.text.active-background",
    HighlightTextActiveForeground => "highlight.text.active-foreground",
    HighlightLineBackground => "highlight.line.background",
    HighlightLineForeground => "highlight.line.foreground",
    IndentGuideBackground => "indent-guide.background",
    IndentGuideActiveBackground => "indent-guide.active-background",
    Link => "link",
    HighlightSearchBackground => "highlight.search.background",
    HighlightSearchForeground => "highlight.search.foreground",
    HighlightButtonBackground => "highlight.button.background",
    HighlightButtonForeground => "highlight.button.foreground",
    SelectionForeground => "selection.foreground",
    SelectionBackground => "selection.background",
    SelectionInactiveBackground => "selection.inactive-background",
    StatusError => "status.error",
    StatusWarning => "status.warning",
    StatusInfo => "status.info",
    StatusSuccess => "status.success",
    TooltipBackground => "tooltip.background",
    TooltipForeground => "tooltip.foreground",
    WhitespaceForeground => "whitespace.foreground",
}

impl fmt::Display for UiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ui {
    pub deprecated: Color,
    pub accent: Color,
    pub border: Color,
    pub cursor: UiCursor,
    pub global: UiGlobal,
    pub gutter: UiBgFg,
    pub highlight: UiHighlight,
    #[serde(rename = "indent-guide")]
    pub indent_guide: UiIndentGuide,
    pub link: Color,
    pub selection: UiSelection,
    pub status: UiStatus,
    pub tooltip: UiBgFg,
    pub whitespace: UiWhitespace,
}

impl Ui {
    #[allow(clippy::too_many_lines)]
    pub fn new(palette: &Palette, variant: &SchemeVariant) -> Self {
        let background = match variant {
            SchemeVariant::Dark => UiGlobalBackground {
                normal: palette.black_normal.clone(),
                dark: palette.black_dim.clone(),
                light: palette.black_bright.clone(),
            },
            SchemeVariant::Light => UiGlobalBackground {
                normal: palette.white_normal.clone(),
                dark: palette.white_dim.clone(),
                light: palette.white_bright.clone(),
            },
        };
        let foreground = match variant {
            SchemeVariant::Dark => UiGlobalForeground {
                normal: palette.white_normal.clone(),
                dark: palette.white_dim.clone(),
                light: palette.white_bright.clone(),
            },
            SchemeVariant::Light => UiGlobalForeground {
                normal: palette.black_normal.clone(),
                dark: palette.black_dim.clone(),
                light: palette.black_bright.clone(),
            },
        };
        let global = UiGlobal {
            background: background.clone(),
            foreground: foreground.clone(),
        };
        let gutter = UiBgFg {
            background: background.normal.clone(),
            foreground: foreground.dark.clone(),
        };
        let highlight = match variant {
            SchemeVariant::Dark => UiHighlight {
                button: UiBgFg {
                    background: palette.black_bright.clone(),
                    foreground: palette.white_normal.clone(),
                },
                line: UiBgFg {
                    background: palette.gray_dim.clone(),
                    foreground: palette.white_dim.clone(),
                },
                text: UiHighlightText {
                    background: palette.gray_dim.clone(),
                    foreground: palette.white_normal.clone(),
                    active_background: palette.gray_normal.clone(),
                    active_foreground: palette.white_normal.clone(),
                },
                search: UiBgFg {
                    background: palette.black_bright.clone(),
                    foreground: palette.yellow_normal.clone(),
                },
            },
            SchemeVariant::Light => UiHighlight {
                button: UiBgFg {
                    background: palette.white_dim.clone(),
                    foreground: palette.black_normal.clone(),
                },
                line: UiBgFg {
                    background: palette.gray_bright.clone(),
                    foreground: palette.black_bright.clone(),
                },
                text: UiHighlightText {
                    background: palette.gray_dim.clone(),
                    foreground: palette.black_normal.clone(),
                    active_background: palette.gray_normal.clone(),
                    active_foreground: palette.black_normal.clone(),
                },
                search: UiBgFg {
                    background: palette.white_dim.clone(),
                    foreground: palette.yellow_normal.clone(),
                },
            },
        };
        let indent_guide = match variant {
            SchemeVariant::Dark => UiIndentGuide {
                background: background.light,
                active_background: palette.gray_dim.clone(),
            },
            SchemeVariant::Light => UiIndentGuide {
                background: background.light,
                active_background: palette.gray_bright.clone(),
            },
        };
        let selection = match variant {
            SchemeVariant::Dark => UiSelection {
                background: palette.black_bright.clone(),
                foreground: palette.white_normal.clone(),
                inactive_background: palette.black_bright.clone(),
            },
            SchemeVariant::Light => UiSelection {
                background: palette.white_dim.clone(),
                foreground: palette.black_normal.clone(),
                inactive_background: palette.white_dim.clone(),
            },
        };
        let accent = palette.cyan_normal.clone();
        let border = palette.gray_dim.clone();
        let cursor = match variant {
            SchemeVariant::Dark => UiCursor {
                normal: foreground.normal.clone(),
                muted: palette.gray_bright.clone(),
            },
            SchemeVariant::Light => UiCursor {
                normal: foreground.normal.clone(),
                muted: palette.gray_dim.clone(),
            },
        };
        let link = palette.cyan_normal.clone();
        let status = UiStatus {
            error: palette.red_normal.clone(),
            info: palette.orange_normal.clone(),
            success: palette.green_normal.clone(),
            warning: palette.yellow_normal.clone(),
        };
        let tooltip = match variant {
            SchemeVariant::Dark => UiBgFg {
                background: palette.black_dim.clone(),
                foreground: foreground.normal,
            },
            SchemeVariant::Light => UiBgFg {
                background: palette.white_bright.clone(),
                foreground: foreground.normal,
            },
        };
        let whitespace = UiWhitespace {
            foreground: palette.gray_normal.clone(),
        };

        Self {
            global,
            deprecated: palette.brown_normal.clone(),
            accent,
            border,
            cursor,
            gutter,
            highlight,
            indent_guide,
            link,
            selection,
            status,
            tooltip,
            whitespace,
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn try_from_basic(
        basic: &BasicUi,
        palette: &Palette,
        variant: &SchemeVariant,
    ) -> Result<Self, TintedBuilderError> {
        let default = Self::new(palette, variant);

        let background = UiGlobalBackground {
            normal: parse_or_inherit(
                &[basic.global_background_normal.as_deref()],
                &default.global.background.normal,
            )?,
            dark: parse_or_inherit(
                &[basic.global_background_dark.as_deref()],
                &default.global.background.dark,
            )?,
            light: parse_or_inherit(
                &[basic.global_background_light.as_deref()],
                &default.global.background.light,
            )?,
        };

        let foreground = UiGlobalForeground {
            normal: parse_or_inherit(
                &[basic.global_foreground_normal.as_deref()],
                &default.global.foreground.normal,
            )?,
            dark: parse_or_inherit(
                &[basic.global_foreground_dark.as_deref()],
                &default.global.foreground.dark,
            )?,
            light: parse_or_inherit(
                &[basic.global_foreground_light.as_deref()],
                &default.global.foreground.light,
            )?,
        };
        let global = UiGlobal {
            background,
            foreground,
        };

        let gutter = UiBgFg {
            background: parse_or_inherit(
                &[basic.gutter_background.as_deref()],
                &default.gutter.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.gutter_foreground.as_deref()],
                &default.gutter.foreground,
            )?,
        };

        let highlight_button = UiBgFg {
            background: parse_or_inherit(
                &[basic.highlight_button_background.as_deref()],
                &default.highlight.button.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.highlight_button_foreground.as_deref()],
                &default.highlight.button.foreground,
            )?,
        };
        let highlight_text = UiHighlightText {
            background: parse_or_inherit(
                &[basic.highlight_text_background.as_deref()],
                &default.highlight.text.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.highlight_text_foreground.as_deref()],
                &default.highlight.text.foreground,
            )?,
            active_background: parse_or_inherit(
                &[basic.highlight_text_active_background.as_deref()],
                &default.highlight.text.active_background,
            )?,
            active_foreground: parse_or_inherit(
                &[basic.highlight_text_active_foreground.as_deref()],
                &default.highlight.text.active_foreground,
            )?,
        };
        let highlight_line = UiBgFg {
            background: parse_or_inherit(
                &[basic.highlight_line_background.as_deref()],
                &default.highlight.line.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.highlight_line_foreground.as_deref()],
                &default.highlight.line.foreground,
            )?,
        };
        let highlight_search = UiBgFg {
            background: parse_or_inherit(
                &[basic.highlight_search_background.as_deref()],
                &default.highlight.search.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.highlight_search_foreground.as_deref()],
                &default.highlight.search.foreground,
            )?,
        };

        let highlight = UiHighlight {
            button: highlight_button,
            line: highlight_line,
            search: highlight_search,
            text: highlight_text,
        };

        let indent_guide = UiIndentGuide {
            background: parse_or_inherit(
                &[basic.indent_guide_background.as_deref()],
                &default.indent_guide.background,
            )?,
            active_background: parse_or_inherit(
                &[basic.indent_guide_active_background.as_deref()],
                &default.indent_guide.active_background,
            )?,
        };

        let selection = UiSelection {
            background: parse_or_inherit(
                &[basic.selection_background.as_deref()],
                &default.selection.background,
            )?,
            foreground: parse_or_inherit(
                &[basic.selection_foreground.as_deref()],
                &default.selection.foreground,
            )?,
            inactive_background: parse_or_inherit(
                &[basic.selection_inactive_background.as_deref()],
                &default.selection.inactive_background,
            )?,
        };

        Ok(Self {
            global,
            deprecated: parse_or_inherit(&[basic.deprecated.as_deref()], &default.deprecated)?,
            accent: parse_or_inherit(&[basic.accent.as_deref()], &default.accent)?,
            border: parse_or_inherit(&[basic.border.as_deref()], &default.border)?,
            cursor: UiCursor {
                normal: parse_or_inherit(
                    &[basic.cursor_normal.as_deref()],
                    &default.cursor.normal,
                )?,
                muted: parse_or_inherit(&[basic.cursor_muted.as_deref()], &default.cursor.muted)?,
            },
            gutter,
            highlight,
            indent_guide,
            link: parse_or_inherit(&[basic.link.as_deref()], &default.link)?,
            selection,
            status: UiStatus {
                error: parse_or_inherit(&[basic.status_error.as_deref()], &default.status.error)?,
                info: parse_or_inherit(&[basic.status_info.as_deref()], &default.status.info)?,
                success: parse_or_inherit(
                    &[basic.status_success.as_deref()],
                    &default.status.success,
                )?,
                warning: parse_or_inherit(
                    &[basic.status_warning.as_deref()],
                    &default.status.warning,
                )?,
            },
            tooltip: UiBgFg {
                background: parse_or_inherit(
                    &[basic.tooltip_background.as_deref()],
                    &default.tooltip.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.tooltip_foreground.as_deref()],
                    &default.tooltip.foreground,
                )?,
            },
            whitespace: UiWhitespace {
                foreground: parse_or_inherit(
                    &[basic.whitespace_foreground.as_deref()],
                    &default.whitespace.foreground,
                )?,
            },
        })
    }

    pub const fn get_color(&self, key: &UiKey) -> &Color {
        match key {
            UiKey::GlobalBackgroundNormal => &self.global.background.normal,
            UiKey::GlobalBackgroundDark => &self.global.background.dark,
            UiKey::GlobalBackgroundLight => &self.global.background.light,
            UiKey::Deprecated => &self.deprecated,
            UiKey::Accent => &self.accent,
            UiKey::Border => &self.border,
            UiKey::CursorNormal => &self.cursor.normal,
            UiKey::CursorMuted => &self.cursor.muted,
            UiKey::GlobalForegroundNormal => &self.global.foreground.normal,
            UiKey::GlobalForegroundDark => &self.global.foreground.dark,
            UiKey::GlobalForegroundLight => &self.global.foreground.light,
            UiKey::GutterBackground => &self.gutter.background,
            UiKey::GutterForeground => &self.gutter.foreground,
            UiKey::HighlightLineBackground => &self.highlight.line.background,
            UiKey::HighlightLineForeground => &self.highlight.line.foreground,
            UiKey::HighlightSearchBackground => &self.highlight.search.background,
            UiKey::HighlightSearchForeground => &self.highlight.search.foreground,
            UiKey::HighlightTextBackground => &self.highlight.text.background,
            UiKey::HighlightTextForeground => &self.highlight.text.foreground,
            UiKey::HighlightTextActiveBackground => &self.highlight.text.active_background,
            UiKey::HighlightTextActiveForeground => &self.highlight.text.active_foreground,
            UiKey::HighlightButtonBackground => &self.highlight.button.background,
            UiKey::HighlightButtonForeground => &self.highlight.button.foreground,
            UiKey::IndentGuideBackground => &self.indent_guide.background,
            UiKey::IndentGuideActiveBackground => &self.indent_guide.active_background,
            UiKey::Link => &self.link,
            UiKey::SelectionForeground => &self.selection.foreground,
            UiKey::SelectionBackground => &self.selection.background,
            UiKey::SelectionInactiveBackground => &self.selection.inactive_background,
            UiKey::StatusError => &self.status.error,
            UiKey::StatusWarning => &self.status.warning,
            UiKey::StatusInfo => &self.status.info,
            UiKey::StatusSuccess => &self.status.success,
            UiKey::TooltipBackground => &self.tooltip.background,
            UiKey::TooltipForeground => &self.tooltip.foreground,
            UiKey::WhitespaceForeground => &self.whitespace.foreground,
        }
    }
}

impl fmt::Display for Ui {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for key in UiKey::variants() {
            writeln!(f, "  {key}: #{}", self.get_color(key).to_hex())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UiGlobal {
    pub background: UiGlobalBackground,
    pub foreground: UiGlobalForeground,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiGlobalBackground {
    pub normal: Color,
    pub dark: Color,
    pub light: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiGlobalForeground {
    pub normal: Color,
    pub dark: Color,
    pub light: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiHighlight {
    pub button: UiBgFg,
    pub line: UiBgFg,
    pub search: UiBgFg,
    pub text: UiHighlightText,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiHighlightText {
    pub background: Color,
    pub foreground: Color,
    #[serde(rename = "active-background")]
    pub active_background: Color,
    #[serde(rename = "active-foreground")]
    pub active_foreground: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiIndentGuide {
    pub background: Color,
    #[serde(rename = "active-background")]
    pub active_background: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiBgFg {
    pub background: Color,
    pub foreground: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiSelection {
    pub background: Color,
    pub foreground: Color,
    #[serde(rename = "inactive-background")]
    pub inactive_background: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiCursor {
    pub normal: Color,
    pub muted: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiStatus {
    pub error: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiWhitespace {
    pub foreground: Color,
}
