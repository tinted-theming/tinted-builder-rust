use serde::Serialize;
use thiserror::Error;

use crate::{
    scheme::tinted8::{yaml::BasicUi, Palette},
    Color,
};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum UiKey {
    BackgroundNormal,
    BackgroundDark,
    BackgroundLight,
    Deprecated,
    Accent,
    Border,
    Cursor,
    ForegroundNormal,
    ForegroundDark,
    ForegroundLight,
    GutterBackground,
    GutterForeground,
    HighlightLineBackground,
    HighlightLineForeground,
    HighlightSearchBackground,
    HighlightSearchForeground,
    HighlightTextBackground,
    HighlightTextForeground,
    HighlightTextActiveBackground,
    HighlightTextActiveForeground,
    IndentGuideBackground,
    IndentGuideActiveBackground,
    Link,
    SelectionForeground,
    SelectionBackground,
    SelectionInactiveBackground,
    StatusError,
    StatusWarning,
    StatusInfo,
    TooltipBackground,
    TooltipForeground,
    WhitespaceForeground,
}

impl UiKey {
    const fn as_str(&self) -> &str {
        match self {
            Self::BackgroundNormal => "background.normal",
            Self::BackgroundDark => "background.dark",
            Self::BackgroundLight => "background.light",
            Self::Deprecated => "deprecated",
            Self::Accent => "accent",
            Self::Border => "border",
            Self::Cursor => "cursor",
            Self::ForegroundNormal => "foreground.normal",
            Self::ForegroundDark => "foreground.dark",
            Self::ForegroundLight => "foreground.light",
            Self::GutterBackground => "gutter.background",
            Self::GutterForeground => "gutter.foreground",
            Self::HighlightTextBackground => "highlight.text.background",
            Self::HighlightTextForeground => "highlight.text.foreground",
            Self::HighlightTextActiveBackground => "highlight.text.active-background",
            Self::HighlightTextActiveForeground => "highlight.text.active-foreground",
            Self::IndentGuideBackground => "indent-guide.background",
            Self::IndentGuideActiveBackground => "indent-guide.active-background",
            Self::HighlightLineBackground => "highlight.line.background",
            Self::HighlightLineForeground => "highlight.line.foreground",
            Self::Link => "link",
            Self::HighlightSearchBackground => "highlight.search.background",
            Self::HighlightSearchForeground => "highlight.search.foreground",
            Self::SelectionForeground => "selection.foreground",
            Self::SelectionBackground => "selection.background",
            Self::SelectionInactiveBackground => "selection.inactive-background",
            Self::StatusError => "status.error",
            Self::StatusWarning => "status.warning",
            Self::StatusInfo => "status.info",
            Self::TooltipBackground => "tooltip.background",
            Self::TooltipForeground => "tooltip.foreground",
            Self::WhitespaceForeground => "whitespace.foreground",
        }
    }

    pub const fn variants() -> &'static [Self] {
        &[
            Self::BackgroundNormal,
            Self::BackgroundDark,
            Self::BackgroundLight,
            Self::Deprecated,
            Self::Accent,
            Self::Border,
            Self::Cursor,
            Self::ForegroundNormal,
            Self::ForegroundDark,
            Self::ForegroundLight,
            Self::GutterBackground,
            Self::GutterForeground,
            Self::HighlightLineBackground,
            Self::HighlightLineForeground,
            Self::HighlightSearchBackground,
            Self::HighlightSearchForeground,
            Self::HighlightTextBackground,
            Self::HighlightTextForeground,
            Self::HighlightTextActiveBackground,
            Self::HighlightTextActiveForeground,
            Self::Link,
            Self::SelectionForeground,
            Self::SelectionBackground,
            Self::SelectionInactiveBackground,
            Self::StatusError,
            Self::StatusWarning,
            Self::StatusInfo,
            Self::TooltipBackground,
            Self::TooltipForeground,
        ]
    }
}

impl fmt::Display for UiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ui {
    pub background: UiBackground,
    pub deprecated: Color,
    pub accent: Color,
    pub border: Color,
    pub cursor: Color,
    pub foreground: UiForeground,
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
        let background = UiBackground {
            normal: palette.black_normal.clone(),
            dark: palette.black_dim.clone(),
            light: palette.black_bright.clone(),
        };
        let foreground = UiForeground {
            normal: palette.white_normal.clone(),
            dark: palette.white_dim.clone(),
            light: palette.white_bright.clone(),
        };
        let gutter = UiBgFg {
            background: background.normal.clone(),
            foreground: foreground.normal.clone(),
        };
        let highlight = UiHighlight {
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
            background: background.light.clone(),
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
            info: palette.cyan_normal.clone(),
            success: palette.green_normal.clone(),
            warning: palette.yellow_normal.clone(),
        };
        let tooltip = UiBgFg {
            background: palette.black_dim.clone(),
            foreground: foreground.normal.clone(),
        };
        let whitespace = UiWhitespace {
            foreground: palette.gray_normal.clone(),
        };

        Self {
            background,
            foreground,
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

        let background = UiBackground {
            normal: basic
                .background_normal
                .map_or_else(
                    || Ok(default.background.normal),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            dark: basic
                .background_dark
                .map_or_else(
                    || Ok(default.background.dark),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            light: basic
                .background_light
                .map_or_else(
                    || Ok(default.background.light),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
        };

        let foreground = UiForeground {
            normal: basic
                .foreground_normal
                .map_or_else(
                    || Ok(default.foreground.normal),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            dark: basic
                .foreground_dark
                .map_or_else(
                    || Ok(default.foreground.dark),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            light: basic
                .foreground_light
                .map_or_else(
                    || Ok(default.foreground.light),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
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
            background,
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
            foreground,
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
            UiKey::BackgroundNormal => &self.background.normal,
            UiKey::BackgroundDark => &self.background.dark,
            UiKey::BackgroundLight => &self.background.light,
            UiKey::Deprecated => &self.deprecated,
            UiKey::Accent => &self.accent,
            UiKey::Border => &self.border,
            UiKey::Cursor => &self.cursor,
            UiKey::ForegroundNormal => &self.foreground.normal,
            UiKey::ForegroundDark => &self.foreground.dark,
            UiKey::ForegroundLight => &self.foreground.light,
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
pub struct UiBackground {
    pub normal: Color,
    pub dark: Color,
    pub light: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiForeground {
    pub normal: Color,
    pub dark: Color,
    pub light: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiHighlight {
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
