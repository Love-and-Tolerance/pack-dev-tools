use pdtlib::comparator::comparator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    comparator(args);
}
