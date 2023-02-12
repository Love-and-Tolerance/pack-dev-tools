use super::pdtfs::find_files_in_dir;
use std::path::{Path, MAIN_SEPARATOR as SLASH};

#[derive(Clone, Copy)]
pub enum CompareType {
    File,
    Hash,
}

pub fn comparator(args: Vec<String>) {
    let mut compare_type: CompareType;
    let compare_file_args = ["-f", "-file"];
    let compare_hash_args = ["-d", "-duplicate"];
    if compare_file_args.contains(&args[1].to_lowercase().as_str()) {
        compare_type = CompareType::File;
    } else if compare_hash_args.contains(&args[1].to_lowercase().as_str()) {
        compare_type = CompareType::Hash;
    } else if Path::new(&args[1]).is_dir() && Path::new(&args[2]).is_dir() {
        compare_type = CompareType::File;
    } else if Path::new(&args[1]).is_file() && Path::new(&args[2]).is_file() {
        compare_type = CompareType::File;
    }
}

pub fn compare_file(dir_one: &str, dir_two: &str, file: &str) -> (bool, bool) {
    let mut dir_one_file = Path::new(&format!("{}{}", &dir_one, &file)).is_file();
    let mut dir_two_file = Path::new(&format!("{}{}", &dir_two, &file)).is_file();
    (dir_one_file, dir_two_file)
}
