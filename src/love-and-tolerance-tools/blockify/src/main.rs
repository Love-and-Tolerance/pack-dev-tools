use clap::{value_parser, Parser};
use pdtlib::blockify::blockify;
use pdtlib::{pdtstdin, pdttrait::Vector};

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = "Blockify images by turning every pixel into a block texture.

example: ./blockify 16 ./assets/minecraft/textures/blocks ./assets",
	long_about = None)
]

struct Args {
	#[arg(value_parser = value_parser!(u32).range(2..=32))]
	/// The width or height of the block textures [2..32]
	block_pixels: u32,
	/// Path to block textures
	blocks_path: String,
	/// List of files and folders to blockify
	input_paths: Vec<String>,
}

fn main() {
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or(Vec::new())
		.extend_vec(args.input_paths);
	blockify(args.block_pixels, args.blocks_path, paths);
}
