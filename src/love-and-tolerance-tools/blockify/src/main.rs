use pdtlib::blockify::blockify;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let blocks_location = args[1].to_string();
    let pack_location = args[2].to_string();
    blockify(blocks_location, pack_location);
}
