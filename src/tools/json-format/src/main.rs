use pdtlib::json_format::{json_formatter, Indent, Json};
use std::any::{Any, TypeId};
use std::env;
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
