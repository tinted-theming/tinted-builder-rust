use serde::Serialize;
use thiserror::Error;

use crate::{
    scheme::tinted8::{yaml::BasicUi, Palette},
    Color,
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
    Cursor => "cursor",
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
    pub cursor: Color,
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
    pub fn new(palette: &Palette) -> Self {
        let background = UiGlobalBackground {
            normal: palette.black_normal.clone(),
            dark: palette.black_dim.clone(),
            light: palette.black_bright.clone(),
        };
        let foreground = UiGlobalForeground {
            normal: palette.white_normal.clone(),
            dark: palette.white_dim.clone(),
            light: palette.white_bright.clone(),
        };
        let global = UiGlobal {
            background: background.clone(),
            foreground: foreground.clone(),
        };
        let gutter = UiBgFg {
            background: background.normal.clone(),
            foreground: foreground.normal.clone(),
        };
        let highlight = UiHighlight {
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
        };
        let indent_guide = UiIndentGuide {
            background: background.light,
            active_background: palette.gray_dim.clone(),
        };
        let selection = UiSelection {
            background: palette.black_bright.clone(),
            foreground: palette.white_normal.clone(),
            inactive_background: palette.black_bright.clone(),
        };
        let accent = palette.blue_normal.clone();
        let border = palette.gray_dim.clone();
        let cursor = foreground.normal.clone();
        let link = palette.cyan_normal.clone();
        let status = UiStatus {
            error: palette.red_normal.clone(),
            info: palette.orange_normal.clone(),
            success: palette.green_normal.clone(),
            warning: palette.yellow_normal.clone(),
        };
        let tooltip = UiBgFg {
            background: palette.black_dim.clone(),
            foreground: foreground.normal,
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
    pub fn try_from_basic(basic: BasicUi, palette: &Palette) -> Result<Self, UiError> {
        let default = Self::new(palette);

        let background = UiGlobalBackground {
            normal: basic
                .global_background_normal
                .map_or_else(
                    || Ok(default.global.background.normal),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            dark: basic
                .global_background_dark
                .map_or_else(
                    || Ok(default.global.background.dark),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            light: basic
                .global_background_light
                .map_or_else(
                    || Ok(default.global.background.light),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        let foreground = UiGlobalForeground {
            normal: basic
                .global_foreground_normal
                .map_or_else(
                    || Ok(default.global.foreground.normal),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            dark: basic
                .global_foreground_dark
                .map_or_else(
                    || Ok(default.global.foreground.dark),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            light: basic
                .global_foreground_light
                .map_or_else(
                    || Ok(default.global.foreground.light),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };
        let global = UiGlobal {
            background,
            foreground,
        };

        let gutter = UiBgFg {
            background: basic
                .gutter_background
                .map_or_else(
                    || Ok(default.gutter.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .gutter_foreground
                .map_or_else(
                    || Ok(default.gutter.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        let highlight_button = UiBgFg {
            background: basic
                .highlight_button_background
                .map_or_else(
                    || Ok(default.highlight.button.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .highlight_button_foreground
                .map_or_else(
                    || Ok(default.highlight.button.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };
        let highlight_text = UiHighlightText {
            background: basic
                .highlight_text_background
                .map_or_else(
                    || Ok(default.highlight.text.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .highlight_text_foreground
                .map_or_else(
                    || Ok(default.highlight.text.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            active_background: basic
                .highlight_text_active_background
                .map_or_else(
                    || Ok(default.highlight.text.active_background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            active_foreground: basic
                .highlight_text_active_foreground
                .map_or_else(
                    || Ok(default.highlight.text.active_foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };
        let highlight_line = UiBgFg {
            background: basic
                .highlight_line_background
                .map_or_else(
                    || Ok(default.highlight.line.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .highlight_line_foreground
                .map_or_else(
                    || Ok(default.highlight.line.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };
        let highlight_search = UiBgFg {
            background: basic
                .highlight_search_background
                .map_or_else(
                    || Ok(default.highlight.search.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .highlight_search_foreground
                .map_or_else(
                    || Ok(default.highlight.search.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        let highlight = UiHighlight {
            button: highlight_button,
            line: highlight_line,
            search: highlight_search,
            text: highlight_text,
        };

        let indent_guide = UiIndentGuide {
            background: basic
                .indent_guide_background
                .map_or_else(
                    || Ok(default.indent_guide.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            active_background: basic
                .indent_guide_active_background
                .map_or_else(
                    || Ok(default.indent_guide.active_background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        let selection = UiSelection {
            background: basic
                .selection_background
                .map_or_else(
                    || Ok(default.selection.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .selection_foreground
                .map_or_else(
                    || Ok(default.selection.foreground),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            inactive_background: basic
                .selection_inactive_background
                .map_or_else(
                    || Ok(default.selection.inactive_background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        Ok(Self {
            global,
            deprecated: basic
                .deprecated
                .map_or_else(|| Ok(default.deprecated), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            accent: basic
                .accent
                .map_or_else(|| Ok(default.accent), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            border: basic
                .border
                .map_or_else(|| Ok(default.border), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            cursor: basic
                .cursor
                .map_or_else(|| Ok(default.cursor), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            gutter,
            highlight,
            indent_guide,
            link: basic
                .link
                .map_or_else(|| Ok(default.link), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            selection,
            status: UiStatus {
                error: basic
                    .status_error
                    .map_or_else(
                        || Ok(default.status.error),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
                info: basic
                    .status_info
                    .map_or_else(
                        || Ok(default.status.info),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
                success: basic
                    .status_success
                    .map_or_else(
                        || Ok(default.status.success),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
                warning: basic
                    .status_warning
                    .map_or_else(
                        || Ok(default.status.warning),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            },
            tooltip: UiBgFg {
                background: basic
                    .tooltip_background
                    .map_or_else(
                        || Ok(default.tooltip.background),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
                foreground: basic
                    .tooltip_foreground
                    .map_or_else(
                        || Ok(default.tooltip.foreground),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            },
            whitespace: UiWhitespace {
                foreground: basic
                    .whitespace_foreground
                    .map_or_else(
                        || Ok(default.whitespace.foreground),
                        |ref s| Color::new(s, None, None),
                    )
                    .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
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
            UiKey::Cursor => &self.cursor,
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

#[derive(Error, Debug)]
pub enum UiError {
    #[error("unable to convert from type: {0}")]
    UnableToConvertFrom(String),
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
