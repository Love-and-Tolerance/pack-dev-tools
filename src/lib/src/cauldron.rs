use super::pdtfs;

pub fn cauldron(color: String, items: Vec<String>) {
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	const EXCLUDE_DIR_NAME: bool = false;
	let files = pdtfs::get_files_in_list(items, recursive, extensions, &EXCLUDE_DIR_NAME);
}
