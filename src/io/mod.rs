mod palette;
mod setting;

use std::path::Path;

pub use setting::Setting;

pub trait ExportExt {
    fn export(&self, path: &Path) -> anyhow::Result<()>;
}

pub trait LoadExt {
    fn load(path: &Path) -> anyhow::Result<Self>
    where
        Self: Sized;
}
