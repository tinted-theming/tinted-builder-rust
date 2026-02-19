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
    ForegroundNormal,
    ForegroundDark,
    ForegroundLight,
    LineBackground,
    LineForeground,
    SearchText,
    SelectionForeground,
    SelectionBackground,
}

impl UiKey {
    const fn as_str(&self) -> &str {
        match self {
            Self::BackgroundNormal => "background.normal",
            Self::BackgroundDark => "background.dark",
            Self::BackgroundLight => "background.light",
            Self::Deprecated => "deprecated",
            Self::ForegroundNormal => "foreground.normal",
            Self::ForegroundDark => "foreground.dark",
            Self::ForegroundLight => "foreground.light",
            Self::LineBackground => "line.background",
            Self::LineForeground => "line.foreground",
            Self::SearchText => "search-text",
            Self::SelectionForeground => "selection.foreground",
            Self::SelectionBackground => "selection.background",
        }
    }

    pub const fn variants() -> &'static [Self] {
        &[
            Self::BackgroundNormal,
            Self::BackgroundDark,
            Self::BackgroundLight,
            Self::Deprecated,
            Self::ForegroundNormal,
            Self::ForegroundDark,
            Self::ForegroundLight,
            Self::LineBackground,
            Self::LineForeground,
            Self::SearchText,
            Self::SelectionForeground,
            Self::SelectionBackground,
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
    pub foreground: UiForeground,
    pub line: UiLine,
    pub search_text: Color,
    pub selection: UiSelection,
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
        let line = UiLine {
            background: palette.gray_dim.clone(),
            foreground: palette.white_dim.clone(),
        };
        let selection = UiSelection {
            background: palette.black_bright.clone(),
            foreground: palette.white_normal.clone(),
        };

        Self {
            background,
            foreground,
            deprecated: palette.brown_normal.clone(),
            search_text: palette.yellow_normal.clone(),
            line,
            selection,
        }
    }
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

        let line = UiLine {
            background: basic
                .line_background
                .map_or_else(
                    || Ok(default.line.background),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground: basic
                .line_foreground
                .map_or_else(
                    || Ok(default.line.foreground),
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
        };

        Ok(Self {
            background,
            deprecated: basic
                .deprecated
                .map_or_else(|| Ok(default.deprecated), |ref s| Color::new(s, None, None))
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            foreground,
            search_text: basic
                .search_text
                .map_or_else(
                    || Ok(default.search_text),
                    |ref s| Color::new(s, None, None),
                )
                .map_err(|err| UiError::UnableToConvertFrom(err.to_string()))?,
            line,
            selection,
        })
    }

    pub const fn get_color(&self, key: &UiKey) -> &Color {
        match key {
            UiKey::BackgroundNormal => &self.background.normal,
            UiKey::BackgroundDark => &self.background.dark,
            UiKey::BackgroundLight => &self.background.light,
            UiKey::Deprecated => &self.deprecated,
            UiKey::ForegroundNormal => &self.foreground.normal,
            UiKey::ForegroundDark => &self.foreground.dark,
            UiKey::ForegroundLight => &self.foreground.light,
            UiKey::LineBackground => &self.line.background,
            UiKey::LineForeground => &self.line.foreground,
            UiKey::SearchText => &self.search_text,
            UiKey::SelectionForeground => &self.selection.foreground,
            UiKey::SelectionBackground => &self.selection.background,
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
pub struct UiLine {
    pub background: Color,
    pub foreground: Color,
}
#[derive(Debug, Clone, Serialize)]
pub struct UiSelection {
    pub background: Color,
    pub foreground: Color,
}
