use super::pdtfs::find_files_in_dir;
use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::fs;

pub enum Json {
    Format,
    Minify,
}

pub enum Indent {
    Tab,
    Space
}

pub fn json_formatter(
    dir: String,
    fmt_type: Json,
    indentifier: Indent,
    indent_number: usize,
) {
    let recursive = true;
    let extensions = Some(vec![".json", ".mcmeta"]);
    let files = find_files_in_dir(&dir, recursive, &extensions);
    let indent = match indentifier {
        Indent::Tab => {
            "\t".repeat(indent_number)
        },
        Indent::Space => {
            " ".repeat(indent_number)
        }
    };
    println!("{indent}");
    for file in files {
        let mut json_data = fs::read_to_string(&file).expect("Failed to read file to string.");
        match fmt_type {
            Json::Format => {
                json_data = format_json(&json_data, &indent);
            }
            Json::Minify => {
                json_data = minify_json(&json_data);
            }
        }
        std::fs::write(file, json_data).expect("Failed to write json to file.");
    }
}

pub fn format_json(json: &str, indent: &str) -> String {
    let value = parse_to_value(json);
    let mut writer = Vec::with_capacity(256);
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut serialiser = Serializer::with_formatter(&mut writer, formatter);
    value
        .serialize(&mut serialiser)
        .expect("Failed to serialize json data.");
    String::from_utf8(writer).expect("Failed to convert utf8 to string.")
}

pub fn minify_json(json: &str) -> String {
    let value = parse_to_value(json);
    serde_json::to_string(&value).expect("Failed to stringify json.")
}

#[inline]
pub fn parse_to_value(json: &str) -> Value {
    serde_json::from_str(json).expect("Failed to parse json.")
}
