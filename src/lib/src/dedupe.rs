use super::pdtfs::{check_if_dir_exists, find_files_in_dir};
use super::pdthash::get_hash;

pub fn dedupe(dir: String) {
    check_if_dir_exists(&dir);
    let recursive = true;
    let extensions = Some(vec![".zip"]);
    let files = find_files_in_dir(&dir, recursive, &extensions);
    let mut records: Vec<(String, String)> = vec![];
    for file in files {
        let hash = get_hash(&file);
        records.push((hash, file));
    }
    records.sort();
    let mut dupes: Vec<String> = vec![];
    let mut i = 0;
    while i < records.len() - 1 {
        if records[i].0 == records[i + 1].0 {
            dupes.push(records[i].1.to_string());
            dupes.push(records[i + 1].1.to_string());
        }
        i += 1;
    }
    dupes.sort();
    dupes.dedup();
    println!("{:#?}", dupes);
}
