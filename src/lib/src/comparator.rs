use super::pdtfs;
use super::pdthash;
use std::path::{Path, MAIN_SEPARATOR as SLASH};

#[derive(Debug)]
pub struct FileData {
    dir_one_presence: bool,
    dir_two_presence: bool,
    filename: String,
    dir_one_hash: Option<String>,
    dir_two_hash: Option<String>,
}

pub fn comparator(args: Vec<String>) {
    let dir_one = pdtfs::check_dir_ends_with_slash(args[1].clone());
    pdtfs::check_if_dir_exists(&dir_one);
    let dir_two = pdtfs::check_dir_ends_with_slash(args[2].clone());
    pdtfs::check_if_dir_exists(&dir_two);
    let recursive = true;
    let dir_one_files = pdtfs::find_files_in_dir(&dir_one, recursive, &None)
        .iter()
        .map(|file| file[dir_one.chars().count()..].to_string())
        .collect::<Vec<String>>();
    let dir_two_files = pdtfs::find_files_in_dir(&dir_two, recursive, &None)
        .iter()
        .map(|file| file[dir_two.chars().count()..].to_string())
        .collect::<Vec<String>>();
    let mut files: Vec<String> = [dir_one_files, dir_two_files].concat();
    files.sort();
    files.dedup();
    let mut results: Vec<FileData> = vec![];
    for file in files {
        results.push(compare_file(&dir_one, &dir_two, file));
    }
    for result in results {
        println!("{result:#?}")
    }
}

pub fn compare_file(dir_one: &str, dir_two: &str, filename: String) -> FileData {
    let dir_one_presence = Path::new(&format!("{}{}", &dir_one, &filename)).is_file();
    let dir_two_presence = Path::new(&format!("{}{}", &dir_two, &filename)).is_file();
    let dir_one_hash = match dir_one_presence {
        true => Some(pdthash::get_hash(
            &format!("{}{}", dir_one, &filename),
            true,
        )),
        false => None,
    };
    let dir_two_hash = match dir_two_presence {
        true => Some(pdthash::get_hash(
            &format!("{}{}", dir_two, &filename),
            true,
        )),
        false => None,
    };
    FileData {
        dir_one_presence,
        dir_two_presence,
        filename,
        dir_one_hash,
        dir_two_hash,
    }
}
