use std::{fs, path::Path};

pub fn check_if_dir_exists(dir: &str) {
    if !Path::new(dir).is_dir() {
        panic!("{dir} not found!");
    }
}

pub fn if_dir_exists_remove_it(dir: &str) {
    if Path::new(dir).is_dir() {
        fs::remove_dir_all(dir).unwrap_or_else(|_| panic!("Failed to remove {dir} directory!"));
    }
}

pub fn if_dir_exists_remove_and_remake_it(dir: &str) {
    if_dir_exists_remove_it(dir);
    fs::create_dir(dir).unwrap_or_else(|_| panic!("Failed to create {dir} directory."));
}

pub fn rename(dir: &str, new_dir: &str) {
    fs::rename(dir, new_dir).unwrap_or_else(|_| panic!("Failed to rename {dir} to {new_dir}."));
}

pub fn find_files_in_dir(
    dir: &str,
    recursive: bool,
    extensions: &Option<Vec<&str>>,
) -> Vec<String> {
    let mut files = vec![];
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.as_ref().unwrap().path().display().to_string();
        if Path::new(&path).is_dir() && recursive {
            files = [files, find_files_in_dir(&path, recursive, extensions)].concat();
        } else if Path::new(&path).is_file() {
            match *extensions {
                Some(ref extensions) => {
                    for ext in extensions {
                        if path.ends_with(ext) {
                            files.push(path.to_string());
                        }
                    }
                }
                None => {
                    files.push(path);
                }
            }
        }
    }
    files
}