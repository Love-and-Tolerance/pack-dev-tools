use camino::Utf8Path;
use fs_extra::dir;
use pony::threads::multithread;
use pony::traits::{BasicVector, OrderedVector};
use std::fs;
use std::path::MAIN_SEPARATOR as SLASH;

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

pub fn find_files_in_multiple_dirs(
	dirs: Vec<String>, recursive: bool, extensions: Option<Vec<String>>, exclude_dir_name: bool,
	announce: bool,
) -> Vec<String> {
	let files = multithread(dirs, None, move |thread_num, dir| {
		if announce {
			println!("[thread {thread_num:02}] finding files in dir: {dir}");
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
		.map(|dir| {
			if !Utf8Path::new(dir).is_dir() {
				panic!("{dir} not found!");
			}
			if dir.ends_with(SLASH) {
				dir.to_string()
			} else {
				format!("{dir}{SLASH}")
			}
		})
		.collect::<Vec<String>>();
	find_files_in_multiple_dirs(dirs, recursive, extensions, exclude_dir_name, announce)
		.extend_vec(files)
		.sort_and_dedup_vec()
}

pub fn copy_files_to_dir(folder: String, items: Vec<String>, content_only: bool) {
	multithread(items, None, move |thread_num, item| {
		println!("[thread {thread_num:02}] copying: {item}");
		if Utf8Path::new(&item).is_dir() {
			let mut options = dir::CopyOptions::new();
			options.content_only = content_only;
			dir::copy(&item, &folder, &options).unwrap();
		} else if Utf8Path::new(&item).is_file() {
			let options = dir::CopyOptions::new();
			fs_extra::copy_items(&[item], &folder, &options).unwrap();
		} else {
			panic!("Entry passed as file or folder not found.");
		}
		None::<()>
	});
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
								.unwrap_or_else(|_| panic!("Failed to remove file: {path}"))
						}
					}
				}
				None => fs::remove_file(path.clone())
					.unwrap_or_else(|_| panic!("Failed to remove file: {path}")),
			}
		}
	}
}
