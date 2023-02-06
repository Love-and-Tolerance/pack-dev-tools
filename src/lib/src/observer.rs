use fs_extra::dir::{copy, CopyOptions};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

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

    if Path::new("./pack/.git").is_dir() {
        fs::remove_dir_all("./pack/.git").expect("Failed to remove old .git directory.");
    }

    #[cfg(target_os = "windows")]
    let dir = Path::new(r".\pack");

    #[cfg(not(target_os = "windows"))]
    let dir = Path::new("./pack");

    assert!(env::set_current_dir(&dir).is_ok());

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "git init"])
        .output()
        .expect("failed to initialize git repo.");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "git add -A"])
        .output()
        .expect("failed to add files.");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "git commit -m \"Initial commit\""])
        .output()
        .expect("failed to commit files.");

    #[cfg(not(target_os = "windows"))]
    Command::new("sh")
        .arg("-c")
        .arg("git init; git add -A; git commit -m \"Initial commit\"")
        .output()
        .expect("failed to initialize git repo.");

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path = path.as_ref().unwrap().path().display().to_string();
        if "./.git" != path {
            if Path::new(&path).is_dir() {
                fs::remove_dir_all(&path).expect(&format!("Failed to remove {} directory.", &path));
            } else if Path::new(&path).is_file() {
                fs::remove_file(&path).expect(&format!("Failed to remove {} file.", &path));
            }
        }
    }

    copy(new_release, "./", &options).expect("Failed to copy new release to pack directory.");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .arg("/C")
        .arg("git add -A")
        .output()
        .expect("failed to add new release.");

    #[cfg(not(target_os = "windows"))]
    Command::new("sh")
        .arg("-c")
        .arg("git add -A")
        .output()
        .expect("failed to add new release.");

    #[cfg(target_os = "windows")]
    let changes = Command::new("cmd")
        .arg("/C")
        .arg("git status -s")
        .output()
        .expect("failed to get changes.");

    #[cfg(not(target_os = "windows"))]
    let changes = Command::new("sh")
        .arg("-c")
        .arg("git status -s")
        .output()
        .expect("failed to get changes.");

    let added: Vec<String>;
    let changed: Vec<String>;
    let renamed: Vec<String>;
    let removed: Vec<String>;

    let changes = String::from_utf8(changes.stdout).expect("Failed to get changes.");
    for change in changes.lines() {
        println!("{change}");
    }
}
