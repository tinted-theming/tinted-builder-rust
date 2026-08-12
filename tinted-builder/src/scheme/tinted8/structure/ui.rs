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
    GlobalNormalBackground => "global.normal.background",
    GlobalNormalForeground => "global.normal.foreground",
    GlobalDarkBackground => "global.dark.background",
    GlobalDarkForeground => "global.dark.foreground",
    GlobalLightBackground => "global.light.background",
    GlobalLightForeground => "global.light.foreground",
    ChromeNormalBackground => "chrome.normal.background",
    ChromeNormalForeground => "chrome.normal.foreground",
    ChromeDarkBackground => "chrome.dark.background",
    ChromeDarkForeground => "chrome.dark.foreground",
    ChromeLightBackground => "chrome.light.background",
    ChromeLightForeground => "chrome.light.foreground",
    Deprecated => "deprecated",
    AccentNormal => "accent.normal",
    BorderNormal => "border.normal",
    CursorNormalBackground => "cursor.normal.background",
    CursorNormalForeground => "cursor.normal.foreground",
    CursorMutedBackground => "cursor.muted.background",
    CursorMutedForeground => "cursor.muted.foreground",
    GutterBackground => "gutter.background",
    GutterForeground => "gutter.foreground",
    HighlightTextBackground => "highlight.text.background",
    HighlightTextForeground => "highlight.text.foreground",
    HighlightTextActiveBackground => "highlight.text.active.background",
    HighlightTextActiveForeground => "highlight.text.active.foreground",
    HighlightLineBackground => "highlight.line.background",
    HighlightLineForeground => "highlight.line.foreground",
    IndentGuideBackground => "indent-guide.background",
    IndentGuideActiveBackground => "indent-guide.active.background",
    LinkNormalBackground => "link.normal.background",
    LinkNormalForeground => "link.normal.foreground",
    HighlightSearchBackground => "highlight.search.background",
    HighlightSearchForeground => "highlight.search.foreground",
    HighlightButtonBackground => "highlight.button.background",
    HighlightButtonForeground => "highlight.button.foreground",
    SelectionForeground => "selection.foreground",
    SelectionBackground => "selection.background",
    SelectionInactiveBackground => "selection.inactive.background",
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
    pub accent: UiAccent,
    pub border: UiBorder,
    pub chrome: UiChrome,
    pub cursor: UiCursor,
    pub global: UiGlobal,
    pub gutter: UiBgFg,
    pub highlight: UiHighlight,
    #[serde(rename = "indent-guide")]
    pub indent_guide: UiIndentGuide,
    pub link: UiLink,
    pub selection: UiSelection,
    pub status: UiStatus,
    pub tooltip: UiBgFg,
    pub whitespace: UiWhitespace,
}

impl Ui {
    #[allow(clippy::too_many_lines)]
    pub fn new(palette: &Palette, variant: &SchemeVariant) -> Self {
        let (bg_normal, bg_dark, bg_light) = match variant {
            SchemeVariant::Dark => (
                palette.black_normal.clone(),
                palette.black_dim.clone(),
                palette.black_bright.clone(),
            ),
            SchemeVariant::Light => (
                palette.white_normal.clone(),
                palette.white_bright.clone(),
                palette.white_dim.clone(),
            ),
        };
        let (fg_normal, fg_dark, fg_light) = match variant {
            SchemeVariant::Dark => (
                palette.white_normal.clone(),
                palette.white_dim.clone(),
                palette.white_bright.clone(),
            ),
            SchemeVariant::Light => (
                palette.black_normal.clone(),
                palette.black_bright.clone(),
                palette.black_dim.clone(),
            ),
        };
        let global = UiGlobal {
            normal: UiBgFg {
                background: bg_normal.clone(),
                foreground: fg_normal.clone(),
            },
            dark: UiBgFg {
                background: bg_dark,
                foreground: fg_dark,
            },
            light: UiBgFg {
                background: bg_light.clone(),
                foreground: fg_light,
            },
        };
        let gutter = UiBgFg {
            background: bg_normal,
            foreground: global.dark.foreground.clone(),
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
                    active: UiBgFg {
                        background: palette.gray_normal.clone(),
                        foreground: palette.white_normal.clone(),
                    },
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
                    background: palette.gray_bright.clone(),
                    foreground: palette.black_normal.clone(),
                    active: UiBgFg {
                        background: palette.gray_normal.clone(),
                        foreground: palette.black_normal.clone(),
                    },
                },
                search: UiBgFg {
                    background: palette.white_dim.clone(),
                    foreground: palette.yellow_normal.clone(),
                },
            },
        };
        let indent_guide = match variant {
            SchemeVariant::Dark => UiIndentGuide {
                background: bg_light,
                active: UiBg {
                    background: palette.gray_dim.clone(),
                },
            },
            SchemeVariant::Light => UiIndentGuide {
                background: bg_light,
                active: UiBg {
                    background: palette.gray_bright.clone(),
                },
            },
        };
        let selection = match variant {
            SchemeVariant::Dark => UiSelection {
                background: palette.black_bright.clone(),
                foreground: palette.white_normal.clone(),
                inactive: UiBg {
                    background: palette.black_bright.clone(),
                },
            },
            SchemeVariant::Light => UiSelection {
                background: palette.white_dim.clone(),
                foreground: palette.black_normal.clone(),
                inactive: UiBg {
                    background: palette.white_dim.clone(),
                },
            },
        };
        let accent = UiAccent {
            normal: palette.cyan_normal.clone(),
        };
        let border = match variant {
            SchemeVariant::Dark => UiBorder {
                normal: palette.gray_dim.clone(),
            },
            SchemeVariant::Light => UiBorder {
                normal: palette.gray_bright.clone(),
            },
        };
        let chrome = match variant {
            SchemeVariant::Dark => UiChrome {
                normal: UiBgFg {
                    background: palette.black_bright.clone(),
                    foreground: palette.white_normal.clone(),
                },
                dark: UiBgFg {
                    background: palette.black_dim.clone(),
                    foreground: palette.white_dim.clone(),
                },
                light: UiBgFg {
                    background: palette.gray_dim.clone(),
                    foreground: palette.white_bright.clone(),
                },
            },
            SchemeVariant::Light => UiChrome {
                normal: UiBgFg {
                    background: palette.white_dim.clone(),
                    foreground: palette.black_normal.clone(),
                },
                dark: UiBgFg {
                    background: palette.gray_bright.clone(),
                    foreground: palette.black_dim.clone(),
                },
                light: UiBgFg {
                    background: palette.white_normal.clone(),
                    foreground: palette.black_bright.clone(),
                },
            },
        };
        let cursor = match variant {
            SchemeVariant::Dark => UiCursor {
                normal: UiBgFg {
                    background: fg_normal.clone(),
                    foreground: global.normal.background.clone(),
                },
                muted: UiBgFg {
                    background: palette.gray_bright.clone(),
                    foreground: palette.gray_dim.clone(),
                },
            },
            SchemeVariant::Light => UiCursor {
                normal: UiBgFg {
                    background: fg_normal.clone(),
                    foreground: global.normal.background.clone(),
                },
                muted: UiBgFg {
                    background: palette.gray_dim.clone(),
                    foreground: palette.gray_bright.clone(),
                },
            },
        };
        let link = UiLink {
            normal: UiBgFg {
                background: global.normal.background.clone(),
                foreground: palette.cyan_normal.clone(),
            },
        };
        let status = UiStatus {
            error: palette.red_normal.clone(),
            info: palette.orange_normal.clone(),
            success: palette.green_normal.clone(),
            warning: palette.yellow_normal.clone(),
        };
        let tooltip = match variant {
            SchemeVariant::Dark => UiBgFg {
                background: palette.black_dim.clone(),
                foreground: fg_normal,
            },
            SchemeVariant::Light => UiBgFg {
                background: palette.white_bright.clone(),
                foreground: fg_normal,
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
            chrome,
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

        let global = UiGlobal {
            normal: UiBgFg {
                background: parse_or_inherit(
                    &[basic.global_normal_background.as_deref()],
                    &default.global.normal.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.global_normal_foreground.as_deref()],
                    &default.global.normal.foreground,
                )?,
            },
            dark: UiBgFg {
                background: parse_or_inherit(
                    &[basic.global_dark_background.as_deref()],
                    &default.global.dark.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.global_dark_foreground.as_deref()],
                    &default.global.dark.foreground,
                )?,
            },
            light: UiBgFg {
                background: parse_or_inherit(
                    &[basic.global_light_background.as_deref()],
                    &default.global.light.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.global_light_foreground.as_deref()],
                    &default.global.light.foreground,
                )?,
            },
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
            active: UiBgFg {
                background: parse_or_inherit(
                    &[basic.highlight_text_active_background.as_deref()],
                    &default.highlight.text.active.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.highlight_text_active_foreground.as_deref()],
                    &default.highlight.text.active.foreground,
                )?,
            },
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
            active: UiBg {
                background: parse_or_inherit(
                    &[basic.indent_guide_active_background.as_deref()],
                    &default.indent_guide.active.background,
                )?,
            },
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
            inactive: UiBg {
                background: parse_or_inherit(
                    &[basic.selection_inactive_background.as_deref()],
                    &default.selection.inactive.background,
                )?,
            },
        };

        let chrome = UiChrome {
            normal: UiBgFg {
                background: parse_or_inherit(
                    &[basic.chrome_normal_background.as_deref()],
                    &default.chrome.normal.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.chrome_normal_foreground.as_deref()],
                    &default.chrome.normal.foreground,
                )?,
            },
            dark: UiBgFg {
                background: parse_or_inherit(
                    &[basic.chrome_dark_background.as_deref()],
                    &default.chrome.dark.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.chrome_dark_foreground.as_deref()],
                    &default.chrome.dark.foreground,
                )?,
            },
            light: UiBgFg {
                background: parse_or_inherit(
                    &[basic.chrome_light_background.as_deref()],
                    &default.chrome.light.background,
                )?,
                foreground: parse_or_inherit(
                    &[basic.chrome_light_foreground.as_deref()],
                    &default.chrome.light.foreground,
                )?,
            },
        };

        Ok(Self {
            global,
            deprecated: parse_or_inherit(&[basic.deprecated.as_deref()], &default.deprecated)?,
            accent: UiAccent {
                normal: parse_or_inherit(
                    &[basic.accent_normal.as_deref()],
                    &default.accent.normal,
                )?,
            },
            border: UiBorder {
                normal: parse_or_inherit(
                    &[basic.border_normal.as_deref()],
                    &default.border.normal,
                )?,
            },
            chrome,
            cursor: UiCursor {
                normal: UiBgFg {
                    background: parse_or_inherit(
                        &[basic.cursor_normal_background.as_deref()],
                        &default.cursor.normal.background,
                    )?,
                    foreground: parse_or_inherit(
                        &[basic.cursor_normal_foreground.as_deref()],
                        &default.cursor.normal.foreground,
                    )?,
                },
                muted: UiBgFg {
                    background: parse_or_inherit(
                        &[basic.cursor_muted_background.as_deref()],
                        &default.cursor.muted.background,
                    )?,
                    foreground: parse_or_inherit(
                        &[basic.cursor_muted_foreground.as_deref()],
                        &default.cursor.muted.foreground,
                    )?,
                },
            },
            gutter,
            highlight,
            indent_guide,
            link: UiLink {
                normal: UiBgFg {
                    background: parse_or_inherit(
                        &[basic.link_normal_background.as_deref()],
                        &default.link.normal.background,
                    )?,
                    foreground: parse_or_inherit(
                        &[basic.link_normal_foreground.as_deref()],
                        &default.link.normal.foreground,
                    )?,
                },
            },
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
            UiKey::GlobalNormalBackground => &self.global.normal.background,
            UiKey::GlobalNormalForeground => &self.global.normal.foreground,
            UiKey::GlobalDarkBackground => &self.global.dark.background,
            UiKey::GlobalDarkForeground => &self.global.dark.foreground,
            UiKey::GlobalLightBackground => &self.global.light.background,
            UiKey::GlobalLightForeground => &self.global.light.foreground,
            UiKey::ChromeNormalBackground => &self.chrome.normal.background,
            UiKey::ChromeNormalForeground => &self.chrome.normal.foreground,
            UiKey::ChromeDarkBackground => &self.chrome.dark.background,
            UiKey::ChromeDarkForeground => &self.chrome.dark.foreground,
            UiKey::ChromeLightBackground => &self.chrome.light.background,
            UiKey::ChromeLightForeground => &self.chrome.light.foreground,
            UiKey::Deprecated => &self.deprecated,
            UiKey::AccentNormal => &self.accent.normal,
            UiKey::BorderNormal => &self.border.normal,
            UiKey::CursorNormalBackground => &self.cursor.normal.background,
            UiKey::CursorNormalForeground => &self.cursor.normal.foreground,
            UiKey::CursorMutedBackground => &self.cursor.muted.background,
            UiKey::CursorMutedForeground => &self.cursor.muted.foreground,
            UiKey::GutterBackground => &self.gutter.background,
            UiKey::GutterForeground => &self.gutter.foreground,
            UiKey::HighlightLineBackground => &self.highlight.line.background,
            UiKey::HighlightLineForeground => &self.highlight.line.foreground,
            UiKey::HighlightSearchBackground => &self.highlight.search.background,
            UiKey::HighlightSearchForeground => &self.highlight.search.foreground,
            UiKey::HighlightTextBackground => &self.highlight.text.background,
            UiKey::HighlightTextForeground => &self.highlight.text.foreground,
            UiKey::HighlightTextActiveBackground => &self.highlight.text.active.background,
            UiKey::HighlightTextActiveForeground => &self.highlight.text.active.foreground,
            UiKey::HighlightButtonBackground => &self.highlight.button.background,
            UiKey::HighlightButtonForeground => &self.highlight.button.foreground,
            UiKey::IndentGuideBackground => &self.indent_guide.background,
            UiKey::IndentGuideActiveBackground => &self.indent_guide.active.background,
            UiKey::LinkNormalBackground => &self.link.normal.background,
            UiKey::LinkNormalForeground => &self.link.normal.foreground,
            UiKey::SelectionForeground => &self.selection.foreground,
            UiKey::SelectionBackground => &self.selection.background,
            UiKey::SelectionInactiveBackground => &self.selection.inactive.background,
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
    pub normal: UiBgFg,
    pub dark: UiBgFg,
    pub light: UiBgFg,
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
    pub active: UiBgFg,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiChrome {
    pub normal: UiBgFg,
    pub dark: UiBgFg,
    pub light: UiBgFg,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiAccent {
    pub normal: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiBorder {
    pub normal: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiLink {
    pub normal: UiBgFg,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiIndentGuide {
    pub background: Color,
    pub active: UiBg,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiBgFg {
    pub background: Color,
    pub foreground: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiBg {
    pub background: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiSelection {
    pub background: Color,
    pub foreground: Color,
    pub inactive: UiBg,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiCursor {
    pub normal: UiBgFg,
    pub muted: UiBgFg,
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
