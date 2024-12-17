use clap::Args;
use palette::Srgb;

use crate::{
    cli::Cli,
    determinator,
    io::{ExportExt, Setting},
    model::{BasePalette, Color, ColorMap, FullPalette, SrgbColorMapExt, ThemeDetectionStrategy},
    optimize::OptimizerExt,
    paths::Paths,
};

#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    pub rgb: Srgb<u8>,

    #[arg(short, long)]
    pub no_saturation_ui: bool,

    // TODO: ここのリネーム
    #[arg(short, long, default_value = "auto")]
    pub color_theme: ThemeDetectionStrategy,
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let determinated = determinator::determinate(args.rgb, args.color_theme);

        // Don't optimize non-colored colors by default, likes Color::Gray
        let pre_optimizing_targets: Vec<_> = Color::colorized_iter().collect();

        let color_map = ColorMap::initialize_with(determinated.colors, &mut rng)
            .optimize(&pre_optimizing_targets, &mut rng);

        let palette = BasePalette::new(determinated.theme, color_map);
        let full_palette = FullPalette::from(&palette);
        let setting = Setting::new(&full_palette, args.no_saturation_ui);

        std::fs::create_dir(Paths::vscode_dir())?;
        palette.export(&Paths::palette())?;
        full_palette.export(&Paths::full_palette())?;
        setting.export(&Paths::setting())?;

        Ok(())
    }
}
