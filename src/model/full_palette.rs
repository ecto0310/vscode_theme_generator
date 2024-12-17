use palette::{FromColor, IntoColor, Lch, Srgb};

use crate::model::{BasePalette, Color, ColorMap, SrgbColorMapExt, Theme};

pub const VARIANTS: usize = 5;
type FullPaletteValue = [Srgb; VARIANTS];

#[derive(Debug, Clone)]
pub struct FullPalette {
    theme: Theme,

    monochrome: FullPaletteValue,
    color_map: ColorMap<FullPaletteValue>,
}

impl FullPalette {
    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn color_map(&self) -> &ColorMap<FullPaletteValue> {
        &self.color_map
    }

    pub fn monochrome(&self) -> &FullPaletteValue {
        &self.monochrome
    }
}

fn make_variant<I: IntoColor<Lch>, O: FromColor<Lch>>(
    i: I,
    theme: Theme,
    double_width: bool,
) -> [O; VARIANTS] {
    let lch: Lch = i.into_color();

    let width_cut = if double_width { 1.0 } else { 2.0 };
    let width = lch.l.min(100.0 - lch.l) / width_cut;

    let mut variants = [
        lch.l + width,
        lch.l + width / 2.0,
        lch.l,
        lch.l - width / 2.0,
        lch.l - width,
    ];

    if theme.is_dark() {
        variants.reverse();
    }

    variants.map(|l| O::from_color(Lch::new(l, lch.chroma, lch.hue)))
}

impl From<&BasePalette> for FullPalette {
    fn from(p: &BasePalette) -> Self {
        let color_map = p
            .color_map()
            .map(|color_key, rgb| make_variant(rgb, p.theme(), color_key == Color::Bg));

        let monochrome_rgb = Srgb::from_color(Lch::new(
            p.color_map().fg_color_avg_luminouse_chroma().0,
            0.0,
            0.0,
        ));

        Self {
            theme: p.theme(),
            monochrome: make_variant(monochrome_rgb, p.theme(), true),
            color_map,
        }
    }
}
