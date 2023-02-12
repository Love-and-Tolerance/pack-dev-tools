use pdtlib::optimize_images::optimize_images;
use std::env;

fn main() {
    let dir = env::args().collect::<Vec<String>>()[1].to_string();
    optimize_images(dir);
}
