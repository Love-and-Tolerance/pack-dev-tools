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

pub enum Structure {
    Ordered,
    Unordered,
}

pub fn comparator(args: Vec<String>) {
    let mut dirs = vec![];
    let mut order = Structure::Ordered;
    let ordered_options = ["-o", "--ordered"];
    let unordered_options = ["-u", "--unordered"];
    if ordered_options.contains(&args[1].to_lowercase().as_str()) {
        dirs = args[2..].to_vec();
    } else if unordered_options.contains(&args[1].to_lowercase().as_str()) {
        dirs = args[2..].to_vec();
        order = Structure::Unordered;
    } else {
        dirs = args[1..].to_vec();
    }
    dirs = dirs
        .iter()
        .map(|d| {
            pdtfs::check_if_dir_exists(&d);
            pdtfs::check_dir_ends_with_slash(d.to_string())
        })
        .collect::<Vec<String>>();

    let recursive = true;
    let files = pdtfs::find_files_in_multiple_dirs(dirs, recursive, &None, &true);
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
