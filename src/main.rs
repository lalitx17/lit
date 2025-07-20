mod commands;
mod utils;

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
    Add {
        file_path: String,
    },
    Commit {
        #[clap(short, long)]
        message: String,
    },
    Log,
    Show {
        hash: String,
    },
    Checkout {
        #[clap(short, long)]
        new_branch: bool,
        branch: Option<String>,
        hash: Option<String>,
    },
    Branch,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Init => commands::init().unwrap(),
        Commands::Add { file_path } => commands::add(&file_path).unwrap(),
        Commands::Commit { message } => commands::commit(&message).unwrap(),
        Commands::Log => println!("{}", commands::log().unwrap()),
        Commands::Show { hash } => match commands::show(&hash).unwrap() {
            commands::show::ShowResult::Exists(content) => println!("{}", content),
            commands::show::ShowResult::NotFound => println!("Object not found"),
        },
        Commands::Checkout {
            hash,
            branch,
            new_branch,
        } => commands::checkout(new_branch, branch, hash).unwrap(),
        Commands::Branch => commands::branch_list().unwrap(),
    }
}
