use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coolpath", about="CoolPath -- 'cause windows sucks")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	#[command(about="list all paths in PATH")]
	List,

	#[command(about="search for path in PATH")]
	Search {
		#[arg(help="needle path to search for")]
		needle: String,
	},

	#[command(about="check if a path is in PATH or not")]
	Exists {
		#[arg(help="path to check")]
		path: String,
	},

	#[command(about="add a new path in PATH")]
	Add {
		#[arg(help="path to add")]
		path: String,
	},

	#[command(about="remove a path from PATH")]
	Remove {
		#[arg(help="path to remove")]
		path: String,
	},
}