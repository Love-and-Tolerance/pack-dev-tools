use super::pdttrait::Vector;
use super::{pdtfs, pdthash};

pub fn dedupe(dir: String) -> Vec<Vec<String>> {
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
