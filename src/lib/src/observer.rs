use super::pdtfs;
use fs_extra::dir::{copy, CopyOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn observe(old_release: String, new_release: String) {
    pdtfs::check_if_dir_exists(&old_release);
    pdtfs::check_if_dir_exists(&new_release);

    #[cfg(target_os = "windows")]
    let slash = r"\";
    #[cfg(not(target_os = "windows"))]
    let slash = "/";

    let observer_dir = format!(".{}observer_dir", &slash);

    pdtfs::if_dir_exists_remove_and_remake_it(&observer_dir);

    let mut options = CopyOptions::new();
    options.content_only = true;
    copy(old_release, &observer_dir, &options)
        .unwrap_or_else(|_| panic!("Failed to copy old release to {} directory.", &observer_dir));

    pdtfs::if_dir_exists_remove_it(&format!("{}{}.git", &observer_dir, &slash));

    assert!(env::set_current_dir(&observer_dir).is_ok());

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

    let paths = fs::read_dir(format!(".{}", &slash)).unwrap();

    for path in paths {
        let path = path.as_ref().unwrap().path().display().to_string();
        if format!(".{}.git", &slash) != path {
            if Path::new(&path).is_dir() {
                fs::remove_dir_all(&path)
                    .unwrap_or_else(|_| panic!("Failed to remove {} directory.", &path));
            } else if Path::new(&path).is_file() {
                fs::remove_file(&path)
                    .unwrap_or_else(|_| panic!("Failed to remove {} file.", &path));
            }
        }
    }

    copy(new_release, ".", &options)
        .unwrap_or_else(|_| panic!("Failed to copy new release to {} directory.", &observer_dir));

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

    let mut added: Vec<String> = vec![];
    let mut changed: Vec<String> = vec![];
    let mut renamed: Vec<String> = vec![];
    let mut removed: Vec<String> = vec![];

    let changes = String::from_utf8(changes.stdout).expect("Failed to get changes.");
    for change in changes.lines() {
        if &change[..1] == "A" {
            added.push(change[3..].to_string());
        } else if &change[..1] == "M" {
            changed.push(change[3..].to_string());
        } else if &change[..1] == "R" {
            renamed.push(change[3..].to_string());
        } else if &change[..1] == "D" {
            removed.push(change[3..].to_string());
        }
    }

    let mut changelog: Vec<String> = vec![];

    changelog.push("## Changelog".to_string());
    changelog.push("".to_string());
    if !added.is_empty() {
        changelog.push("### Added".to_string());
        changelog.push("".to_string());
        for change in added {
            changelog.push(format!("- `{change}`"));
        }
        changelog.push("".to_string());
    }
    if !changed.is_empty() {
        changelog.push("### Changed".to_string());
        changelog.push("".to_string());
        for change in changed {
            changelog.push(format!("- `{change}`"));
        }
        changelog.push("".to_string());
    }
    if !renamed.is_empty() {
        changelog.push("### Renamed / Moved".to_string());
        changelog.push("".to_string());
        for change in renamed {
            let old_name = change.split("->").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();
            let new_name = change.split("->").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();
            changelog.push(format!("- `{old_name}` -> `{new_name}`"));
        }
        changelog.push("".to_string());
    }
    if !removed.is_empty() {
        changelog.push("### Removed".to_string());
        changelog.push("".to_string());
        for change in removed {
            changelog.push(format!("- `{change}`"));
        }
        changelog.push("".to_string());
    }
    changelog.push("".to_string());

    let mut file = fs::File::create(format!(".{}changelog.md", &slash))
        .expect("Failed to create changelog file.");

    file.write_all(changelog.join("\n").as_bytes())
        .expect("Failed to write changelog file.");
}
