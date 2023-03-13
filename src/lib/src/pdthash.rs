use sha2::{Digest, Sha256};
use std::{fs, io};

use super::pdtthread;

pub fn get_hashes(files: Vec<String>) -> Vec<(String, String)> {
	pdtthread::multithread(files, None, |thread_num, file| {
		println!(
			"[thread {thread_num:02}] getting hash of file: {}",
			file.split('/').last().unwrap()
		);

		let hash = get_hash(&file, false);
		Some((hash, file))
	})
}

pub fn get_hash(filename: &str, announce: bool) -> String {
	if announce {
		println!("Getting hash of file: {}", &filename);
	}
	let mut file = fs::File::open(filename).expect("Failed to open file.");
	let mut hasher = Sha256::new();
	io::copy(&mut file, &mut hasher).expect("Failed to copy file into writer.");
	let hash = hasher.finalize();
	format!("{hash:x}")
}
