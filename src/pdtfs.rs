use super::pdtthread;
use super::pdttrait::Vector;
use camino::Utf8Path;
use fs_extra::dir;
use std::fs;
use std::path::MAIN_SEPARATOR as SLASH;

pub fn check_if_dir_exists(dir: &str) {
	if !Utf8Path::new(dir).is_dir() {
		panic!("{dir} not found!");
	}
}

pub fn if_dir_exists_remove_it(dir: &str) {
	if Utf8Path::new(dir).is_dir() {
		fs::remove_dir_all(dir).unwrap_or_else(|_| panic!("Failed to remove {dir} directory!"));
	}
}

pub fn if_dir_exists_remove_and_remake_it(dir: &str) {
	if_dir_exists_remove_it(dir);
	fs::create_dir_all(dir).unwrap_or_else(|_| panic!("Failed to create {dir} directory."));
}

pub fn rename(dir: &str, new_dir: &str) {
	fs::rename(dir, new_dir).unwrap_or_else(|_| panic!("Failed to rename {dir} to {new_dir}."));
}

pub fn find_files_in_dir(
	dir: &str, recursive: bool, extensions: &Option<Vec<String>>,
) -> Vec<String> {
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into()).unwrap();
	for path in paths {
		let path = path.unwrap().path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files = [files, find_files_in_dir(&path, recursive, extensions)].concat();
		} else if utf8_path.is_file() {
			match *extensions {
				Some(ref extensions) => {
					for ext in extensions {
						if path.ends_with(ext) {
							files.push(path.to_string());
						}
					}
				}
				None => {
					files.push(path);
				}
			}
		}
	}
	files
}

pub fn check_dir_ends_with_slash(dir: String) -> String {
	if dir.ends_with(SLASH) {
		dir
	} else {
		format!("{dir}{SLASH}")
	}
}

pub fn find_files_in_multiple_dirs(
	dirs: Vec<String>, recursive: bool, extensions: Option<Vec<String>>, exclude_dir_name: bool,
	announce: bool,
) -> Vec<String> {
	let files = pdtthread::multithread(dirs, None, move |thread_num, dir| {
		if announce {
			println!("[thread {thread_num:02}] finding files in dir: {}", dir);
		}
		let dir_files = if exclude_dir_name {
			find_files_in_dir(&dir, recursive, &extensions)
				.iter()
				.map(|f| f[dir.chars().count()..].to_string())
				.collect()
		} else {
			find_files_in_dir(&dir, recursive, &extensions)
		};

		Some(dir_files)
	});

	files.into_iter().flatten().collect()
}

pub fn get_files_in_list(
	items: Vec<String>, recursive: bool, extensions: Option<Vec<String>>, exclude_dir_name: bool,
	announce: bool,
) -> Vec<String> {
	let files = items
		.iter()
		.filter(|f| Utf8Path::new(f).is_file() && f.ends_with(".png"))
		.map(|f| f.to_string())
		.collect::<Vec<String>>();
	let dirs = items
		.iter()
		.filter(|f| Utf8Path::new(f).is_dir())
		.map(|d| {
			check_if_dir_exists(d);
			check_dir_ends_with_slash(d.to_string())
		})
		.collect::<Vec<String>>();
	find_files_in_multiple_dirs(dirs, recursive, extensions, exclude_dir_name, announce)
		.extend_vec(files)
		.sort_and_dedup_vec()
}

pub fn create_output_dir(name: &str) -> String {
	if_dir_exists_remove_and_remake_it(name);
	format!(".{SLASH}{name}")
}

pub fn copy_files_to_dir(folder: String, items: Vec<String>, content_only: bool) {
	pdtthread::multithread(items, None, move |thread_num, item| {
		println!("[thread {thread_num:02}] copying: {}", item);
		if Utf8Path::new(&item).is_dir() {
			copy_dir_to_dir(&folder, item, content_only);
		} else if Utf8Path::new(&item).is_file() {
			copy_file_to_dir(&folder, item);
		} else {
			panic!("Entry passed as file or folder not found.");
		}
		None::<()>
	});
}

pub fn copy_dir_to_dir(output: &String, input: String, content_only: bool) {
	let mut options = dir::CopyOptions::new();
	options.content_only = content_only;
	dir::copy(&input, output, &options).unwrap_or_else(|_| {
		panic!(
			"Failed to copy {} directory to {} directory.",
			&input, &output
		)
	});
}

pub fn copy_file_to_dir(output: &String, input: String) {
	let options = dir::CopyOptions::new();
	fs_extra::copy_items(&[&input], output, &options)
		.unwrap_or_else(|_| panic!("Failed to copy {} file to {} directory.", &input, &output));
}

pub fn find_dirs_in_dir(dir: &str, recursive: bool) -> Vec<String> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into()).unwrap();
	for path in paths {
		let path = path.unwrap().path().to_string();
		if Utf8Path::new(&path).is_dir() {
			dirs.push(path.clone());
			if recursive {
				dirs = [dirs, find_dirs_in_dir(&path, recursive)].concat();
			}
		}
	}
	dirs
}

pub fn delete_files_in_dir(dir: &str, recursive: bool, extensions: &Option<Vec<String>>) {
	let paths = Utf8Path::read_dir_utf8(dir.into()).unwrap();
	for path in paths {
		let path = path.unwrap().path().to_string();
		if Utf8Path::new(&path).is_dir() && recursive {
			delete_files_in_dir(&path, recursive, extensions);
		} else if Utf8Path::new(&path).is_file() {
			match *extensions {
				Some(ref extensions) => {
					for ext in extensions {
						if path.ends_with(ext) {
							fs::remove_file(path.clone())
								.unwrap_or_else(|_| panic!("Failed to remove file: {}", path))
						}
					}
				}
				None => fs::remove_file(path.clone())
					.unwrap_or_else(|_| panic!("Failed to remove file: {}", path)),
			}
		}
	}
}
