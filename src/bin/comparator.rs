use camino::Utf8Path;
use pdt::pdttrait::Vector;
use pdt::{pdtfs, pdthash, pdtthread};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut dirs: Vec<String>;
	let mut order = Structure::Ordered;
	let ordered_options = ["-o", "--ordered"];
	let unordered_options = ["-u", "--unordered"];
	if ordered_options.contains(&args[1].to_lowercase().as_str()) {
		dirs = args[2..].to_vec();
	} else if unordered_options.contains(&args[1].to_lowercase().as_str()) {
		dirs = args[2..].to_vec();
		order = Structure::Unordered;
	} else {
		dirs = args[1..].to_vec();
	}
	dirs = dirs
		.iter()
		.map(|d| {
			pdtfs::check_if_dir_exists(d);
			pdtfs::check_dir_ends_with_slash(d.to_string())
		})
		.collect::<Vec<String>>();

	let recursive = true;
	match order {
		Structure::Ordered => {}
		Structure::Unordered => {}
	}
	let files = pdtfs::find_files_in_multiple_dirs(dirs.clone(), recursive, None, true, false)
		.sort_and_dedup_vec();
	let file_data = get_files_data(dirs.clone(), files);
	let results = compare_files(dirs, file_data);
	let mut changes: Vec<String> = vec![];
	for result in &results {
		let changed = result.presence_version.clone().sort_and_dedup_vec().len() > 1;
		if changed {
			changes.push(format!(
				"{} {}",
				result
					.presence_version
					.iter()
					.map(|&id| id.to_string())
					.collect::<Vec<_>>()
					.join(" "),
				result.filename
			));
		}
	}
	for change in &changes {
		println!("{}", change);
	}
	eprintln!("{}", &changes.len());
}

#[derive(Debug)]
struct FileData {
	filename: String,
	folder_hash: Vec<Option<String>>,
}

#[derive(Debug)]
struct PresenceData {
	filename: String,
	presence_version: Vec<usize>,
}

enum Structure {
	Ordered,
	Unordered,
}

fn get_files_data(dirs: Vec<String>, files: Vec<String>) -> Vec<FileData> {
	pdtthread::multithread(files, None, move |_, file| {
		let dir_data = dirs
			.iter()
			.map(|dir| {
				let presence = Utf8Path::new(&format!("{}{}", &dir, &file)).is_file();
				match presence {
					true => Some(pdthash::get_hash(&format!("{}{}", dir, file), false)),
					false => None,
				}
			})
			.collect::<Vec<Option<String>>>();

		Some(FileData {
			filename: file,
			folder_hash: dir_data,
		})
	})
}

fn compare_files(dirs: Vec<String>, files: Vec<FileData>) -> Vec<PresenceData> {
	pdtthread::multithread(files, None, move |_, file| {
		let mut presence_data: Vec<usize> = vec![];
		for i in 0..dirs.len() {
			let mut id = 0;
			if let Some(hash) = &file.folder_hash[i] {
				if file.folder_hash[0..i].contains(&Some(hash.to_string())) {
					id = file.folder_hash[0..i]
						.iter()
						.filter(|h| h.is_some())
						.collect::<Vec<_>>()
						.iter()
						.position(|h| h == &&Some(hash.to_string()))
						.unwrap() + 1;
				} else {
					let hash_array = file.folder_hash[0..i]
						.iter()
						.filter_map(|h| h.clone())
						.collect::<Vec<_>>()
						.sort_and_dedup_vec();

					id = hash_array.len() + 1;
				}
				presence_data.push(id);
			} else {
				presence_data.push(id);
			}
		}

		Some(PresenceData {
			filename: file.filename,
			presence_version: presence_data,
		})
	})
}
