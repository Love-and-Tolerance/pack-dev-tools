use pdtlib::json_fmt::{json_formatter, Indent, Json};
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
                if args[i + 2].parse::<u8>().unwrap().type_id() == TypeId::of::<u8>() {
                    indent = Indent::Space(args[i + 2].parse::<u8>().unwrap());
                    i += 3;
                    continue;
                } else {
                    indent = Indent::Space(2);
                }
            }
        } else if minify_args.contains(&args[i].to_lowercase().as_str()) {
            fmt_type = Json::Minify;
        }
        i += 1;
    }
}
