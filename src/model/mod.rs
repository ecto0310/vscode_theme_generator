mod hex_str;
pub use hex_str::HexStr;

mod color;
pub use color::Color;

mod color_map;
pub use color_map::{ColorMap, SrgbColorMapExt};

mod base_palette;
pub use base_palette::BasePalette;

mod full_palette;
pub use full_palette::{FullPalette, VARIANTS as FULL_PALETTE_VARIANTS};

mod scored_value;
pub use scored_value::{Scoreable, ScoredValue};

mod linear;
pub use linear::Linear;

mod theme;
pub use theme::{Theme, ThemeDetectionStrategy};
