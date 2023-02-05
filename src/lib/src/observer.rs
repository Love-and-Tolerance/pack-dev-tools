use fs_extra::dir::{copy, CopyOptions};
use git2::{IndexAddOption, Repository};
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

    let mut options = CopyOptions::new();
    options.content_only = true;
    copy(old_release, "./pack/", &options).expect("Failed to copy old release to pack directory.");

    let repo = match Repository::init("./pack") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let mut index = repo.index().expect("Failed to get index");

    index
        .add_all(["*"].iter(), IndexAddOption::FORCE, None)
        .expect("Failed to add files to git.");

    index
        .write()
        .expect("Failed to write all files to the index.");

    let sig = repo.signature().expect("Failed to get signature.");

    let tree_id = {
        let mut index = repo.index().expect("Failed to get index.");
        index.write_tree().expect("Failed to write tree.")
    };

    let tree = repo.find_tree(tree_id).expect("Failed to get tree.");

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .expect("Failed to commit.");
}
