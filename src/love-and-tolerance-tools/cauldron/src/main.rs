use clap::{value_parser, Parser};
use pdtlib::cauldron::cauldron;
use pdtlib::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Tint png images to a specified color and saturation.

example: .{s}cauldron F5B7D0 .{s}images
example: .{s}cauldron e6c343 -s 65 .{s}assets", s = SLASH),
	long_about = None)
]

struct Args {
	/// Hex color to tint images
	hex_color: String,
	#[arg(short,long,value_parser = value_parser!(u32).range(0..=100))]
	/// Saturation level [0..100]
	saturation: Option<u32>,
	/// List of files and folders to tint
	input_paths: Vec<String>,
}

fn main() {
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or(Vec::new())
		.extend_vec(args.input_paths);
	let saturation: Option<f32> = match args.saturation {
		Some(_) => Some(args.saturation.unwrap() as f32),
		None => None,
	};
	cauldron(args.hex_color, paths, saturation);
}
