use pdtlib::blockify::blockify;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let blocks_location = args
        .get(1)
        .expect("first arg should be location of assets/minecraft/textures/block");
    let pack_location = args
        .get(2)
        .expect("second arg should be location of assets (dir in a pack)");
    let optimise = args.get(3).is_some();
    blockify(blocks_location.into(), pack_location.into(), optimise);
}
