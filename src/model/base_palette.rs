use crate::{
    determinator,
    foreign::SrgbExt,
    model::{Color, ColorMap, SrgbColorMapExt, Theme},
    optimize::OptimizerExt,
};

use palette::Srgb;

#[derive(Debug, Clone)]
pub struct BasePalette {
    theme: Theme,
    color_map: ColorMap<Srgb>,
}

impl BasePalette {
    pub fn new(theme: Theme, color_map: ColorMap<Srgb>) -> Self {
        Self { theme, color_map }
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn regenerate_colors<R: rand::Rng>(&self, targets: &[Color], rng: &mut R) -> Self {
        let mut color_map = self.color_map;
        let base = self.color_map.base_color();

        for &target in targets {
            if target.is_bg_color() {
                let d = determinator::determinate(base, self.theme.into());
                color_map[target] = d.colors.bg;
            } else {
                color_map[target] = base.new_by_random_hue(rng);
            }
        }

        Self::new(self.theme, color_map)
    }

    pub fn color_map(&self) -> &ColorMap<Srgb> {
        &self.color_map
    }
}

impl OptimizerExt for BasePalette {
    fn optimize<R: rand::Rng>(self, targets: &[Color], rng: &mut R) -> Self {
        Self {
            theme: self.theme,
            color_map: self.color_map.optimize(targets, rng),
        }
    }
}
