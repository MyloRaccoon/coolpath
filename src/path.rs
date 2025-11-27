use std::env;
use winreg::enums::*;
use winreg::RegKey;

pub fn get_paths() -> Vec<String> {
	env::var("PATH").expect("You're PATH doesn't exists. Oh well. You're fucked.").split(';').map(|p| String::from(p)).collect()
}

pub fn list() {
	println!("~ Current PATH");
	for path in get_paths() {
		println!("   ~ {path}");
	}
}

pub fn search(needle: String) {
	println!("~ Search Result");
	for path in get_paths() {
		if path.contains(&needle) {
			println!("   ~ {path}");
		}
	}
}

pub fn path_exists(path: &String) -> bool {
	get_paths().contains(path)
}

pub fn add(path: String) {

	if path_exists(&path) {
		eprintln!("Error: path '{path}' already in PATH.");
		return;
	}
	if path.is_empty() {
		eprintln!("Error: arg 'path' can't be empty.");
		return;
	}

	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
	let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE).unwrap();

	let old_path: String = env.get_value("Path").unwrap();

	let new_path = format!("{};{}", old_path, path);

	env.set_value("Path", &new_path).expect("Error: couldn't update PATH."); 

	println!("Added value {path} to PATH");
}

pub fn remove(path: String) {

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

	let paths_str = paths.join(";");
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
	let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE).unwrap();

	env.set_value("Path", &paths_str).expect("Error: couldn't update PATH.");

	println!("Removed value {path} from PATH");
}