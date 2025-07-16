mod commands;

use clap::{Parser, Subcommand};

#[derive(clap::Parser)]
#[clap(author = "Lalit", version = "1.0", about = "A simple Git-like tool")]
struct Cli {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add,
    Commit,
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Init => commands::init(),
        Commands::Add => commands::add(),
        Commands::Commit => commands::commit(),
        Commands::Log => commands::log(),
    }
}
