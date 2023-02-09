use pdtlib::json_fmt::{json_formatter, Json, Indent};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if Path::new(&args[1]).is_dir() {
        json_formatter(args[1].to_owned(), Json::Format, Indent::Space, 5);
    }
}
