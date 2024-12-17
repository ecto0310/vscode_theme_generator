use std::path::PathBuf;

pub struct Paths;

impl Paths {
    pub fn vscode_dir() -> PathBuf {
        PathBuf::from(".vscode")
    }

    pub fn palette() -> PathBuf {
        Self::vscode_dir().join("palette.json")
    }

    pub fn full_palette() -> PathBuf {
        Self::vscode_dir().join("full_palette.json")
    }

    pub fn setting() -> PathBuf {
        Self::vscode_dir().join("settings.json")
    }
}
