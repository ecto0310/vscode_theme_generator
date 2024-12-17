use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb};

pub trait SrgbExt {
    fn compare(&self, other: &Self) -> f32;
    fn new_by_random_hue<R: rand::Rng>(&self, rng: &mut R) -> Self;
}

impl SrgbExt for Srgb {
    fn compare(&self, other: &Self) -> f32 {
        Lch::from_color(*self).difference(Lch::from_color(*other))
    }

    fn new_by_random_hue<R: rand::Rng>(&self, rng: &mut R) -> Self {
        let hue = rng.gen_range(0.0..360.0);
        let base_lch = Lch::from_color(*self);
        Self::from_color(Lch::new(base_lch.l, base_lch.chroma, hue))
    }
}
