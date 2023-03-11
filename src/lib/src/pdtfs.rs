use super::pdtthread;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR as SLASH};
use std::sync::{Arc, Mutex};

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
        let path = path.unwrap().path().to_str().unwrap().to_string();
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

pub fn check_dir_ends_with_slash(dir: String) -> String {
    if dir.ends_with(SLASH) {
        dir
    } else {
        format!("{dir}{SLASH}")
    }
}

pub fn find_files_in_multiple_dirs(
    dirs: Vec<String>,
    recursive: bool,
    extensions: &'static Option<Vec<&str>>,
    exclude_dir_name: &'static bool,
) -> Vec<String> {
    let files = Arc::new(Mutex::new(Vec::new()));
    let dirs = dirs.into_iter().map(|d| (d, Arc::clone(&files))).collect();

    pdtthread::multithread(dirs, None, move |thread_num, (dir, files)| {
        println!("[thread {thread_num:02}] finding files in dir: {}", dir);
        let dir_files = if *exclude_dir_name {
            find_files_in_dir(&dir, recursive, &extensions)
                .iter()
                .map(|f| f[dir.chars().count()..].to_string())
                .collect()
        } else {
            find_files_in_dir(&dir, recursive, &extensions)
        };
        dir_files
            .iter()
            .for_each(|f| files.lock().unwrap().push(f.to_string()))
    });
    Arc::try_unwrap(files).ok().unwrap().into_inner().unwrap()
}
