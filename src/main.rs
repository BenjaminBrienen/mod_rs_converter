use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
	let project_dir = std::env::args().nth(1).expect("Usage: mod_rs_converter <project_dir>");
	convert_mod_rs_structure(&Path::new(&project_dir)).expect("Conversion failed");
}

/// Traverse the directory structure and process `mod.rs` files
fn convert_mod_rs_structure(base_dir: &Path) -> std::io::Result<()> {
	for entry in WalkDir::new(base_dir).into_iter().filter_map(Result::ok) {
		if entry.file_type().is_file() && entry.file_name() == "mod.rs" {
			let mod_rs_path = entry.path().to_path_buf();
			if let Some(new_path) = get_new_module_path(&mod_rs_path) {
				move_and_rename_mod_rs(&mod_rs_path, &new_path)?;
			}
		}
	}
	Ok(())
}

/// Given the path to `mod.rs`, determine the new path based on the containing directory
fn get_new_module_path(mod_rs_path: &Path) -> Option<PathBuf> {
	let grandparent = mod_rs_path.parent()?.parent()?;
	let file_name = mod_rs_path.parent()?.file_name()?;
	let mut new_path = grandparent.join(file_name);
	new_path.set_extension("rs");
	Some(new_path)
}

/// Move `mod.rs` to the new path and rename it
fn move_and_rename_mod_rs(old_path: &Path, new_path: &Path) -> std::io::Result<()> {
	println!("Renaming and moving {} to {}", old_path.display(), new_path.display());

	// Ensure destination doesn't exist to avoid overwrite issues
	if new_path.exists() {
		println!("Warning: Destination file {} already exists. Skipping.", new_path.display());
		return Ok(());
	}

	fs::rename(old_path, new_path)?;
	Ok(())
}
