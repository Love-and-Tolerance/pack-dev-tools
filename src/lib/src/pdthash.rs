use sha2::{Digest, Sha256};
use std::{fs, io};

pub fn get_hash(filename: &str) -> String {
    println!("Getting hash of file: {}", &filename);
    let mut file = fs::File::open(filename).expect("Failed to open file.");
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).expect("Failed to copy file into writer.");
    let hash = hasher.finalize();
    format!("{hash:x}")
}
