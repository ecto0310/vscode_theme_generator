use palette::{FromColor, Lch, Srgb};

use crate::{
    foreign::SrgbExt,
    model::{Theme as T, ThemeDetectionStrategy as S},
};

const BLACK: Srgb = Srgb::new(0.0, 0.0, 0.0);
const WHITE: Srgb = Srgb::new(1.0, 1.0, 1.0);

#[derive(Debug, Clone, Copy)]
pub struct DeterminatedColors {
    pub bg: Srgb,
    pub reference: Srgb,
}

#[derive(Debug, Clone, Copy)]
pub struct Determinated {
    pub theme: T,
    pub colors: DeterminatedColors,
}

pub fn determinate<C: Into<Srgb>>(color: C, strategy: S) -> Determinated {
    let color: Srgb = color.into();
    let color_lch = Lch::from_color(color);
    let mid = Srgb::from_color(Lch::new(50.0, 50.0, color_lch.hue));
    let darken = Srgb::from_color(Lch::new(10.0, 10.0, color_lch.hue));
    let whiten = Srgb::from_color(Lch::new(95.0, 5.0, color_lch.hue));

    let (theme, bg, reference, is_default_theme) = if BLACK.compare(&color) < 10.5 {
        match strategy {
            S::Auto | S::Dark => (T::Dark, color, mid, false),
            S::Light => (T::Light, whiten, mid, false),
        }
    } else if WHITE.compare(&color) < 10.5 {
        match strategy {
            S::Dark => (T::Dark, darken, mid, false),
            S::Auto | S::Light => (T::Light, color, mid, false),
        }
    } else {
        let theme = match strategy {
            S::Auto if 42.0 < darken.compare(&color) => T::Dark,
            S::Auto | S::Light => T::Light,
            S::Dark => T::Dark,
        };

        match theme {
            T::Dark => (theme, darken, color, true),
            T::Light => (theme, whiten, color, true),
        }
    };

    println!("determinated as {theme} (default: {is_default_theme}) theme");

    Determinated {
        theme,
        colors: DeterminatedColors { bg, reference },
    }
}
