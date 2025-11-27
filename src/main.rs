mod cli;
mod path;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Some(command) => match command {
			Commands::List => path::list(),
			Commands::Search { needle } => path::search(&needle),
			Commands::Exists { path } => println!(
				"{}",
				if path::path_exists(&path) {
					"YES"
				} else {
					"NO"
				}
			),
			Commands::Add { path } => path::add(&path),
			Commands::Remove { path } => (), //path::remove(&path),
		},
		None => path::list(),
	}
}
