use clap::Args;

use crate::{
    io::{ExportExt, LoadExt, Setting},
    model::{BasePalette, Color, FullPalette},
    optimize::OptimizerExt,
    paths::Paths,
    Cli,
};

#[derive(Debug, Clone, Args)]
pub struct RegenerateArgs {
    #[clap(short, long)]
    pub fixs: Vec<Color>,

    #[arg(short, long)]
    pub no_saturation_ui: bool,
}

impl Cli {
    pub fn regenerate(args: &RegenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let palette = BasePalette::load(&Paths::palette())?
            .regenerate_colors(&args.fixs, &mut rng)
            .optimize(&args.fixs, &mut rng);

        let full_palette = FullPalette::from(&palette);

        let setting = Setting::new(&full_palette, args.no_saturation_ui);

        palette.export(&Paths::palette())?;
        full_palette.export(&Paths::full_palette())?;
        setting.export(&Paths::setting())?;

        Ok(())
    }
}
