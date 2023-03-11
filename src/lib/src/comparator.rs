use super::{pdtfs, pdthash, pdtthread};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct FileData {
    filename: String,
    folder_hash: Vec<Option<String>>,
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
    let files = pdtfs::find_files_in_multiple_dirs(dirs.clone(), recursive, &None, &true);
    let file_data = get_files_data(dirs, files);
    println!("{}", file_data.len());
}

pub fn get_files_data(dirs: Vec<String>, files: Vec<String>) -> Vec<FileData> {
    pdtthread::multithread(files, None, move |thread_num, file| {
        println!(
            "[thread {thread_num:02}] getting information for file: {}",
            file.split('/').last().unwrap()
        );

        let dir_data = dirs
            .iter()
            .map(|dir| {
                let presence = Path::new(&format!("{}{}", &dir, &file)).is_file();
                match presence {
                    true => Some(pdthash::get_hash(&format!("{}{}", dir, file), false)),
                    false => None,
                }
            })
            .collect::<Vec<Option<String>>>();

        Some(FileData {
            filename: file,
            folder_hash: dir_data,
        })
    })
}
