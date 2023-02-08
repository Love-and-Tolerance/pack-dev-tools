use super::{pdtcmd, pdtfs};
use fs_extra::dir::{copy, CopyOptions};
use std::io::Write;
use std::path::Path;
use std::process::Output;
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
    {
        pdtcmd::execute_windows_command("git init");
        pdtcmd::execute_windows_command("git add -A");
        pdtcmd::execute_windows_command("git commit -m \"Initial commit\"");
    }

    #[cfg(not(target_os = "windows"))]
    {
        pdtcmd::execute_unix_command("git init");
        pdtcmd::execute_unix_command("git add -A");
        pdtcmd::execute_unix_command("git commit -m \"Initial commit\"");
    }

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

    if Path::new(&format!("{}{}.git", &new_release, &slash)).is_dir()
        || Path::new(&format!("{}.git", &new_release)).is_dir()
    {
        pdtfs::rename(
            &format!(".{}.git", &slash),
            &format!(".{}.git_temp", &slash),
        );
    }

    copy(new_release, ".", &options)
        .unwrap_or_else(|_| panic!("Failed to copy new release to {} directory.", &observer_dir));

    if Path::new(&format!(".{}.git_temp", &slash)).is_dir() {
        pdtfs::if_dir_exists_remove_it(&format!(".{}.git", &slash));
        pdtfs::rename(
            &format!(".{}.git_temp", &slash),
            &format!(".{}.git", &slash),
        );
    }

    let changes: Output;

    #[cfg(target_os = "windows")]
    {
        pdtcmd::execute_windows_command("git add -A");
        changes = pdtcmd::execute_windows_command_with_return("git status -s");
    }

    #[cfg(not(target_os = "windows"))]
    {
        pdtcmd::execute_unix_command("git add -A");
        changes = pdtcmd::execute_unix_command_with_return("git status -s");
    }

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
