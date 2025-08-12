use camino::Utf8Path;
use pdt::pdtfs;
use pony::command::{execute_command, execute_command_with_return};
use std::error::Error;
use std::io::Write;
use std::path::MAIN_SEPARATOR as SLASH;
use std::process::Output;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = env::args().collect();
	let old_release = args[1].to_string();
	let new_release = args[2].to_string();
	observe(old_release, new_release)?;
	Ok(())
}

fn observe(old_release: String, new_release: String) -> Result<(), Box<dyn Error>> {
	pdtfs::check_if_dir_exists(&old_release);
	pdtfs::check_if_dir_exists(&new_release);

	if !execute_command("git --version")?.success() {
		return Err("git not installed".into());
	}

	let observer_dir = pdtfs::create_output_dir("observer_output");

	pdtfs::if_dir_exists_remove_and_remake_it(&observer_dir);

	pdtfs::copy_dir_to_dir(&observer_dir, old_release, true);

	pdtfs::if_dir_exists_remove_it(&format!("{}{}.git", &observer_dir, SLASH));

	assert!(env::set_current_dir(&observer_dir).is_ok());

	execute_command("git init")?;
	execute_command("git add -A")?;
	execute_command("git commit -m \"Initial commit\"")?;

	let paths = fs::read_dir(format!(".{SLASH}")).unwrap();

	for path in paths {
		let path = path.as_ref().unwrap().path().display().to_string();
		if format!(".{SLASH}.git") != path {
			if Utf8Path::new(&path).is_dir() {
				fs::remove_dir_all(&path)
					.unwrap_or_else(|_| panic!("Failed to remove {} directory.", &path));
			} else if Utf8Path::new(&path).is_file() {
				fs::remove_file(&path)
					.unwrap_or_else(|_| panic!("Failed to remove {} file.", &path));
			}
		}
	}

	if Utf8Path::new(&format!("{}{}.git", &new_release, SLASH)).is_dir()
		|| Utf8Path::new(&format!("{}.git", &new_release)).is_dir()
	{
		pdtfs::rename(&format!(".{SLASH}.git"), &format!(".{SLASH}.git_temp"));
	}

	pdtfs::copy_dir_to_dir(&".".to_string(), new_release, true);

	if Utf8Path::new(&format!(".{SLASH}.git_temp")).is_dir() {
		pdtfs::if_dir_exists_remove_it(&format!(".{SLASH}.git"));
		pdtfs::rename(&format!(".{SLASH}.git_temp"), &format!(".{SLASH}.git"));
	}

	execute_command("git add -A")?;
	let changes: Output = execute_command_with_return("git status -s")?;

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

	let mut file = fs::File::create(format!(".{SLASH}changelog.md"))
		.expect("Failed to create changelog file.");

	file.write_all(changelog.join("\n").as_bytes())
		.expect("Failed to write changelog file.");

	Ok(())
}
