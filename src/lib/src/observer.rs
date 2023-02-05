use std::fs;
use std::path::Path;

pub fn observe(old_release: String, new_release: String) {
    if !Path::new(&old_release).is_dir() {
        panic!("Old release directory not found!")
    }
    if !Path::new(&new_release).is_dir() {
        panic!("New release directory not found!")
    }
    if Path::new("./pack").is_dir() {
        fs::remove_dir_all("./pack").expect("Failed to remove pack directory.");
    }
    fs::create_dir("./pack").expect("Failed to create pack directory.");
}
