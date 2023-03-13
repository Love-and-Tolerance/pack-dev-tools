use super::pdtfs;
use super::pdttrait::Vector;
use std::path::Path;

pub fn cauldron(color: String, items: Vec<String>) {
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	let files = items
		.iter()
		.filter(|f| Path::new(f).is_file())
		.map(|f| f.to_string())
		.collect::<Vec<String>>();
	let dirs = items
		.iter()
		.filter(|f| Path::new(f).is_dir())
		.map(|d| {
			pdtfs::check_if_dir_exists(d);
			pdtfs::check_dir_ends_with_slash(d.to_string())
		})
		.collect::<Vec<String>>();
	let files = pdtfs::find_files_in_multiple_dirs(dirs, recursive, extensions, &false)
		.extend_vec(files)
		.sort_and_dedup_vec();
}
