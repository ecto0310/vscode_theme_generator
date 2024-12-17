use clap::ValueEnum;
use enum_iterator::Sequence;
use linearize::Linearize;
use serde::{Deserialize, Serialize};

/// Base Palette
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Linearize, Sequence, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    // Fg is always automatically calculated by FullPalette
    Bg,
    Gray,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Pink,
}

impl Color {
    pub fn colorized_iter() -> impl Iterator<Item = Self> {
        enum_iterator::all::<Self>().filter(|v| v.is_colorized())
    }

    pub fn is_bg_color(&self) -> bool {
        *self == Self::Bg
    }

    pub fn is_colorized(&self) -> bool {
        !matches!(self, Self::Bg | Self::Gray)
    }
}
