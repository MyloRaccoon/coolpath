use std::{env, fmt::Display, process::exit};
use winreg::{
	RegKey,
	enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
	path: String,
}

pub struct Paths {
	paths: Vec<Path>,
}

// impl Paths {

// }

impl Path {
	pub fn new(value: &str) -> Self {
		Self {
			path: value.to_string(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.path.is_empty()
	}

	pub fn contains(&self, pattern: &str) -> bool {
		self.path.contains(pattern)
	}
}

impl Display for Path {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.path)
	}
}

pub fn get_paths() -> Vec<Path> {
	let paths = env::var("PATH");

	match paths {
		Ok(res) => {
			let mut paths = vec![];
			for path_str in res.split(';') {
				let path = Path::new(path_str);
				if !path.is_empty() {
					println!("WARNING: {path} is not a real path");
				} else if path.is_empty() {
					println!("WARNING: a path is empty");
				}
				paths.push(path);
			}
			paths
		}
		Err(e) => {
			eprintln!("{e}");
			exit(1)
		}
	}
}

pub fn list() {
	println!("~ Current PATH");
	for path in get_paths() {
		println!("   ~ {path}");
	}
}

pub fn search(needle: &str) {
	println!("~ Search Result");
	for path in get_paths() {
		if path.contains(needle) {
			println!("   ~ {path}");
		}
	}
}

pub fn path_exists(path: &str) -> bool {
	get_paths().contains(&Path::new(path))
}

pub fn add(path: &str) {
	if path_exists(path) {
		eprintln!("Error: path '{path}' already in PATH.");
		return;
	}
	if path.is_empty() {
		eprintln!("Error: arg 'path' can't be empty.");
		return;
	}

	let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
	let env = hkey_current_user.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE);

	let env = match env {
		Ok(env) => env,
		Err(e) => {
			eprintln!("{e}");
			exit(1)
		}
	};

	let old_path: String = match env.get_value("Path") {
		Ok(old_path) => old_path,
		Err(e) => {
			eprintln!("{e}");
			exit(1)
		}
	};

	let new_path = format!("{old_path};{path}");

	env.set_value("Path", &new_path)
		.expect("Error: couldn't update PATH.");

	println!("Added value {path} to PATH");
}

pub fn remove(path: &str) {
	let path = Path::new(path);

	if path.is_empty() {
		eprintln!("Error: arg 'path' can't be empty.");
		return;
	}

	let mut paths = get_paths();
	if let Some(pos) = paths.iter().position(|p| *p == path) {
		paths.remove(pos);
	} else {
		eprintln!("Error: path '{path}' not in PATH.");
		return;
	}

	// let paths_str = paths.join(";"); // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH
	let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
	let env = hkey_current_user.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE);

	let env = match env {
		Ok(env) => env,
		Err(e) => {
			eprintln!("{e}");
			exit(1)
		}
	};

	env.set_value("Path", &paths_str)
		.expect("Error: couldn't update PATH.");

	println!("Removed value {path} from PATH");
}
