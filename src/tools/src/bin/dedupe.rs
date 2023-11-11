use pdtlib::pdttrait::Vector;
use pdtlib::{pdtfs, pdthash};
use std::path::MAIN_SEPARATOR as SLASH;
use std::time::SystemTime;
use std::{env, fs, io::Write};

fn main() {
	let args: Vec<String> = env::args().collect();
	let dir = args[1].to_string();
	let dupes = dedupe(dir.clone());
	let mut file_data: Vec<String> = vec![];
	file_data.push(format!("duplicate files in {dir}"));
	file_data.push("".to_string());
	for set in dupes {
		for item in set {
			file_data.push(item);
		}
		file_data.push("".to_string());
	}

	let timestamp = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.expect("Failed to get timestamp.");

	let timestamp = format!("{timestamp:?}").split('.').collect::<Vec<&str>>()[0].to_string();

	let mut file = fs::File::create(format!(".{SLASH}duplicate_files_{timestamp}.txt"))
		.expect("Failed to create duplicates file.");

	file.write_all(file_data.join("\n").as_bytes())
		.expect("Failed to write duplicates file.");
}

fn dedupe(dir: String) -> Vec<Vec<String>> {
	pdtfs::check_if_dir_exists(&dir);
	let recursive = true;
	let extensions = Some(vec![".zip".to_string()]);
	let files = pdtfs::find_files_in_dir(&dir, recursive, &extensions);
	let records = pdthash::get_hashes(files).sort_vec();
	let mut dupes: Vec<Vec<String>> = vec![];
	let mut i = 0;
	while i < records.len() {
		let dupe: Vec<String> = records
			.iter()
			.filter(|x| records[i].0 == x.0)
			.map(|x| x.1.to_string())
			.collect();
		if dupe.len() > 1 {
			dupes.push(dupe.clone());
		}
		i += dupe.len();
	}
	dupes.sort_vec()
}
