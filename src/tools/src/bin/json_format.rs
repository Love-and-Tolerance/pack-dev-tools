use camino::Utf8Path;
use pdtlib::pdtfs;
use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::any::{Any, TypeId};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
	let mut fmt_type = Json::Format;
	let mut indent = Indent::Tab;
	let format_args = ["-f", "-format"];
	let minify_args = ["-m", "-minify"];
	let tab_indent_args = ["-t", "-tab"];
	let space_indent_args = ["-s", "-space"];
	let args: Vec<String> = env::args().collect();
	let mut i = 1;
	while i < args.len() {
		if Path::new(&args[i]).is_dir() || Path::new(&args[i]).is_file() {
			json_formatter(args[i].to_owned(), fmt_type, indent);
		} else if format_args.contains(&args[i].to_lowercase().as_str()) {
			fmt_type = Json::Format;
			if tab_indent_args.contains(&args[i + 1].to_lowercase().as_str()) {
				indent = Indent::Tab;
			} else if space_indent_args.contains(&args[i + 1].to_lowercase().as_str()) {
				(i, indent) = parse_space_indent(i, args.clone(), 2);
			}
		} else if minify_args.contains(&args[i].to_lowercase().as_str()) {
			fmt_type = Json::Minify;
		} else if tab_indent_args.contains(&args[i].to_lowercase().as_str()) {
			indent = Indent::Tab;
		} else if space_indent_args.contains(&args[i].to_lowercase().as_str()) {
			(i, indent) = parse_space_indent(i, args.clone(), 1);
		}
		i += 1;
	}
}

fn parse_space_indent(mut i: usize, args: Vec<String>, num: usize) -> (usize, Indent) {
	let indent: Indent;
	if args[i + 1].parse::<usize>().is_ok()
		&& args[i + num].parse::<usize>().unwrap().type_id() == TypeId::of::<usize>()
	{
		indent = Indent::Space(args[i + num].parse::<u8>().unwrap_or_else(|_| {
			panic!("Failed to parse to u8."); // help go here.
		}));
		if let Indent::Space(num) = indent {
			if !(1..=16).contains(&num) {
				panic!("Num of spaces out of bounds."); // help go here.
			}
		}
		i += num;
	} else {
		indent = Indent::Space(2);
	}
	(i, indent)
}

#[derive(Copy, Clone)]
pub enum Json {
	Format,
	Minify,
}

#[derive(Copy, Clone)]
enum Indent {
	Tab,
	Space(u8),
}

fn json_formatter(dir_or_file: String, fmt_type: Json, indent: Indent) {
	let recursive = true;
	let extensions = Some(vec![".json".to_string(), ".mcmeta".to_string()]);
	let mut files = vec![];
	if Utf8Path::new(&dir_or_file).is_dir() {
		files = pdtfs::find_files_in_dir(&dir_or_file, recursive, &extensions);
	} else if Utf8Path::new(&dir_or_file).is_file() {
		files.push(dir_or_file);
	}
	let indent: String = match indent {
		Indent::Tab => "\t".to_string(),
		Indent::Space(indent_number) => " ".repeat(indent_number as usize),
	};
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

fn format_json(json: &str, indent: &str) -> String {
	let value = parse_to_value(json);
	// json.len() is not ideal but its a _goodish_ default
	let mut writer = Vec::with_capacity(json.len());
	let formatter = PrettyFormatter::with_indent(indent.as_bytes());
	let mut serialiser = Serializer::with_formatter(&mut writer, formatter);
	value
		.serialize(&mut serialiser)
		.expect("Failed to serialize json data.");
	writer.push(b'\n');
	String::from_utf8(writer).expect("Failed to convert utf8 to string.")
}

fn minify_json(json: &str) -> String {
	let value = parse_to_value(json);
	serde_json::to_string(&value).expect("Failed to stringify json.")
}

#[inline]
fn parse_to_value(json: &str) -> Value {
	serde_json::from_str(json).expect("Failed to parse json.")
}
