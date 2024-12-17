use clap::{Parser, Subcommand};
use generate::GenerateArgs;
use regenerate::RegenerateArgs;

pub mod generate;
pub mod regenerate;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Generate(GenerateArgs),
    Regenerate(RegenerateArgs),
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Generate(args) => Self::generate(args),
            Commands::Regenerate(args) => Self::regenerate(args),
        }
    }
}
