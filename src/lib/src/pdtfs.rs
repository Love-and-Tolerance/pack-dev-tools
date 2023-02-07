use std::{fs, path::Path};

pub fn check_if_dir_exists(dir: &str) {
    if !Path::new(&dir).is_dir() {
        panic!("{dir} not found!");
    }
}

pub fn if_dir_exists_remove_it(dir: &str) {
    if Path::new(&dir).is_dir() {
        fs::remove_dir_all(dir).unwrap_or_else(|_| panic!("Failed to remove {dir} directory!"));
    }
}

pub fn if_dir_exists_remove_and_remake_it(dir: &str) {
    if_dir_exists_remove_it(&dir);
    fs::create_dir(&dir).unwrap_or_else(|_| panic!("Failed to create {} directory.", &dir));
}
