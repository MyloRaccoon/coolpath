use clap::Parser;
use coolpath::{path, cli::{Cli, Commands}};

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            match command {
                Commands::List => path::list(),
                Commands::Search { needle } => path::search(needle),
                Commands::Exists { path } => println!("{}", match path::path_exists(&path) { true => "YES", false => "NO" }),
                Commands::Add { path } => path::add(path),
                Commands::Remove { path } => path::remove(path),
            }
        },
        None => path::list(),
    };
}
