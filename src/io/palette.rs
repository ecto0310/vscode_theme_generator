use std::fs::File;
use std::io::Write;
use std::path::Path;

use palette::Srgba;
use serde::{Deserialize, Serialize};

use crate::{
    io::{ExportExt, LoadExt},
    model::{BasePalette, ColorMap, FullPalette, HexStr, Theme, FULL_PALETTE_VARIANTS},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BasePaletteExportable {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_map: ColorMap<HexStr>,
}

type HexColors = [HexStr; FULL_PALETTE_VARIANTS];

#[derive(Debug, Clone, Deserialize, Serialize)]
struct FullPaletteExportable {
    #[serde(rename = "$schema")]
    schema: String,
    dark: bool,

    #[serde(flatten)]
    color_map: ColorMap<HexColors>,

    // rename to 'fg' (historical reasons)
    #[serde(rename = "fg")]
    monochrome: HexColors,
}

impl From<&BasePalette> for BasePaletteExportable {
    fn from(v: &BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.theme().is_dark(),
            color_map: v.color_map().map_values(|v| HexStr(Srgba::from(v).into()))
        }
    }
}

impl From<&FullPalette> for FullPaletteExportable {
    fn from(v: &FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.theme().is_dark(),
            monochrome: v.monochrome().map(|c| HexStr(Srgba::from(c).into())),
            color_map: v.color_map().map_values(|v| v.map(|v| HexStr(Srgba::from(v).into()))),
        }
    }
}

impl LoadExt for BasePalette {
    fn load(path: &Path) -> anyhow::Result<Self> {
        let palette = std::fs::read_to_string(path)?;
        let palette: BasePaletteExportable = serde_json::from_str(&palette)?;

        let theme = Theme::from_is_dark(palette.dark);
        let color_map = palette.color_map.map_values(|v| v.0.color.into());

        Ok(Self::new(theme, color_map))
    }
}

impl ExportExt for BasePalette {
    fn export(&self, path: &Path) -> anyhow::Result<()> {
        let palette = BasePaletteExportable::from(self);
        let palette = serde_json::to_string_pretty(&palette)?;
        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }
}

impl ExportExt for FullPalette {
    fn export(&self, path: &Path) -> anyhow::Result<()> {
        let palette = FullPaletteExportable::from(self);
        let palette = serde_json::to_string_pretty(&palette)?;
        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }
}
