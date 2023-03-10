use sha2::{Digest, Sha256};
use std::{
    fs, io,
    sync::{Arc, Mutex},
};

use super::pdtthread;

pub fn get_hashes(files: Vec<String>) -> Vec<(String, String)> {
    let records = Arc::new(Mutex::new(Vec::new()));
    let files = files
        .into_iter()
        .map(|f| (f, Arc::clone(&records)))
        .collect();

    pdtthread::multithread(files, None, |thread_num, (file, records)| {
        println!(
            "[thread {thread_num:02}] getting hash of file: {}",
            file.split('/').last().unwrap()
        );
        let hash = get_hash(&file, false);
        records.lock().unwrap().push((hash, file))
    });
    Arc::try_unwrap(records).unwrap().into_inner().unwrap()
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
