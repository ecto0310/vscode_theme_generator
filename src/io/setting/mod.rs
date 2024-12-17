use std::{fs::File, io::Write, path::Path};

use palette::Srgba;
use serde::Serialize;

mod editor_token_color_customizations;
mod text_mate_rule;
mod workbench_color_customizations;

use editor_token_color_customizations::EditorTokenColorCustomizations;
use workbench_color_customizations::WorkbenchColorCustomizations;

use super::ExportExt;
use crate::model::{Color, FullPalette, HexStr, FULL_PALETTE_VARIANTS};

#[derive(Serialize)]
pub struct Setting {
    #[serde(rename = "workbench.colorCustomizations")]
    workbench_color_customizations: WorkbenchColorCustomizations,

    #[serde(rename = "editor.tokenColorCustomizations")]
    editor_token_color_customizations: EditorTokenColorCustomizations,
}

type HexColors = [HexStr; FULL_PALETTE_VARIANTS];

impl Setting {
    pub fn new(palette: &FullPalette, force_monochrome_ui_texts: bool) -> Self {
        let color_map = palette
            .color_map()
            .map_values(|v| v.map(|v| HexStr(Srgba::from(v).into())));

        let monochrome = palette.monochrome().map(|v| HexStr(Srgba::from(v).into()));

        let ui_gray = if force_monochrome_ui_texts {
            monochrome
        } else {
            color_map[Color::Gray]
        };

        Self {
            workbench_color_customizations: WorkbenchColorCustomizations::new(&color_map, ui_gray),
            editor_token_color_customizations: EditorTokenColorCustomizations::new(&color_map),
        }
    }
}

impl ExportExt for Setting {
    fn export(&self, path: &Path) -> anyhow::Result<()> {
        let setting = serde_json::to_string_pretty(self)?;
        File::create(path)?.write_all(setting.as_bytes())?;
        Ok(())
    }
}
