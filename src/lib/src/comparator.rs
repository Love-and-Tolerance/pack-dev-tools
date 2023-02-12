use super::pdtfs::{check_if_dir_exists, find_files_in_dir};
use super::pdthash::get_hash;
use std::path::{Path, MAIN_SEPARATOR as SLASH};

pub struct FileData {
    dir_one_presence: bool,
    dir_two_presence: bool,
    filename: String,
    dir_one_hash: Option<String>,
    dir_two_hash: Option<String>,
}

pub fn comparator(args: Vec<String>) {
    let dir_one = args[1].as_str();
    check_if_dir_exists(dir_one);
    let dir_two = args[2].as_str();
    check_if_dir_exists(dir_two);
    let recursive = true;
    let dir_one_files = find_files_in_dir(dir_one, recursive, &None);
    let dir_two_files = find_files_in_dir(dir_two, recursive, &None);
    let results: Vec<FileData> = vec![];
}

pub fn compare_file(dir_one: &str, dir_two: &str, file: &str) -> FileData {
    let dir_one_file = Path::new(&format!("{}{}", &dir_one, &file)).is_file();
    let dir_two_file = Path::new(&format!("{}{}", &dir_two, &file)).is_file();
    FileData {
        dir_one_presence: dir_one_file,
        dir_two_presence: dir_two_file,
        filename: file.to_string(),
        dir_one_hash: Some("test".to_string()),
        dir_two_hash: Some("test".to_string()),
    }
}
