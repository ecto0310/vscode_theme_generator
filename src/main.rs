use clap::Parser;
use cli::Cli;

mod cli;
mod determinator;
mod foreign;
mod io;
mod model;
mod optimize;
mod paths;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
