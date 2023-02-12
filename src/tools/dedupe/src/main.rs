use pdtlib::{dedupe, pdtos};
use std::time::SystemTime;
use std::{env, fs, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args[1].to_string();
    let dupes = dedupe::dedupe(dir.clone());
    let mut file_data: Vec<String> = vec![];
    file_data.push(format!("duplicate files in {dir}"));
    file_data.push("".to_string());
    for set in dupes {
        for item in set {
            file_data.push(item);
        }
        file_data.push("".to_string());
    }

    let slash = pdtos::get_os_slash();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get timestamp.");

    let timestamp = format!("{timestamp:?}").split('.').collect::<Vec<&str>>()[0].to_string();

    let mut file = fs::File::create(format!(".{}duplicate_files_{}.txt", &slash, timestamp))
        .expect("Failed to create duplicates file.");

    file.write_all(file_data.join("\n").as_bytes())
        .expect("Failed to write duplicates file.");
}
