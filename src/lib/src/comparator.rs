use super::pdtfs::{check_if_dir_exists, find_files_in_dir, check_dir_ends_with_slash};
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
    let dir_one = check_dir_ends_with_slash(args[1].clone());
    check_if_dir_exists(&dir_one);
    let dir_two = check_dir_ends_with_slash(args[2].clone());
    check_if_dir_exists(&dir_two);
    let recursive = true;
    let dir_one_files = find_files_in_dir(&dir_one, recursive, &None);
    let dir_two_files = find_files_in_dir(&dir_two, recursive, &None);
    let results: Vec<FileData> = vec![];
}

pub fn compare_file(dir_one: &str, dir_two: &str, filename: String) -> FileData {
    let dir_one_presence = Path::new(&format!("{}{}", &dir_one, &filename)).is_file();
    let dir_two_presence = Path::new(&format!("{}{}", &dir_two, &filename)).is_file();
    let mut dir_one_hash: Option<String> = None;
    let mut dir_two_hash: Option<String> = None;
    if dir_one_presence && dir_two_presence {
        dir_one_hash = Some(get_hash(&format!("{}{}", dir_one, &filename)));
        dir_two_hash = Some(get_hash(&format!("{}{}", dir_two, &filename)));
    }
    FileData {
        dir_one_presence,
        dir_two_presence,
        filename,
        dir_one_hash,
        dir_two_hash,
    }
}
