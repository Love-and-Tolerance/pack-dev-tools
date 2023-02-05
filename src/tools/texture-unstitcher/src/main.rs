use pdtlib::texture_unstitch::unstitch_texture;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].to_string();
    let width = args[2].to_string().parse::<u32>().unwrap();
    let height = args[3].to_string().parse::<u32>().unwrap();
    unstitch_texture(filename, width, height);
}
