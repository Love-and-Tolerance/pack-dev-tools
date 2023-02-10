use pdtlib::dedupe::dedupe;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args[1].to_string();
    dedupe(dir);
}
