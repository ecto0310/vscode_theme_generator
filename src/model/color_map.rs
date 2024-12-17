use linearize::{static_copy_map, StaticCopyMap};
use palette::{FromColor, Lch, Srgb};

use crate::{determinator::DeterminatedColors, foreign::SrgbExt, model::Color};

pub type ColorMap<T> = StaticCopyMap<Color, T>;

pub trait SrgbColorMapExt {
    fn initialize_with<R: rand::Rng>(color: DeterminatedColors, rng: &mut R) -> Self;
    fn base_color(&self) -> Srgb;
    fn fg_color_avg_luminouse_chroma(&self) -> (f32, f32);
}

impl SrgbColorMapExt for ColorMap<Srgb> {
    fn initialize_with<R: rand::Rng>(color: DeterminatedColors, rng: &mut R) -> Self {
        static_copy_map! {
            Color::Bg => color.bg,
            Color::Gray => color.reference,
            _ => color.reference.new_by_random_hue(rng),
        }
    }

    fn base_color(&self) -> Srgb {
        let (l, chroma) = self.fg_color_avg_luminouse_chroma();
        let bg = Lch::from_color(self[Color::Bg]);

        Srgb::from_color(Lch::new(l, chroma, bg.hue))
    }

    fn fg_color_avg_luminouse_chroma(&self) -> (f32, f32) {
        let n = enum_iterator::all::<Color>()
            .filter(|c| !c.is_bg_color())
            .count() as f32;

        self.iter()
            .filter_map(|(k, v)| (!k.is_bg_color()).then_some(v))
            .map(|c| Lch::from_color(*c))
            .map(|c| (c.l / n, c.chroma / n))
            .fold((0.0, 0.0), |(l_acc, c_acc), (l, c)| (l_acc + l, c_acc + c))
    }
}
