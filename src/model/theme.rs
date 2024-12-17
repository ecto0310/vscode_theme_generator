use clap::ValueEnum;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn is_dark(&self) -> bool {
        *self == Self::Dark
    }

    pub fn from_is_dark(b: bool) -> Self {
        match b {
            true => Self::Dark,
            false => Self::Light,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Light => write!(f, "light"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ThemeDetectionStrategy {
    Auto,
    Dark,
    Light,
}

impl From<Theme> for ThemeDetectionStrategy {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Dark => Self::Dark,
            Theme::Light => Self::Light,
        }
    }
}
