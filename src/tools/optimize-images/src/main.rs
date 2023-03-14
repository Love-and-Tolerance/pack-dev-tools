use clap::{value_parser, Parser};
use pdtlib::optimize_images::{optimize_images, Strip};
use pdtlib::{pdtstdin, pdttrait::Vector};

#[derive(Debug, Parser)]
#[command(name = "optimize_images", bin_name = "optimize_images", version)]
struct Args {
	/// Compression level [possible values: 0 - 6]
	#[arg(short, long, default_value_t = 6, value_parser = value_parser!(u8).range(0..=6))]
	level: u8,
	#[arg(short, long, value_enum, default_value_t = Strip::Safe)]
	strip: Strip,
	/// Try to fix errors when decoding the input file
	#[arg(long, short)]
	fix: bool,
	/// Enable Adam7 interlacing
	#[arg(long, short)]
	interlace: bool,
	/// List of files and folders to optimize
	paths: Vec<String>,
}

fn main() {
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or(Vec::new())
		.extend_vec(args.paths);
	optimize_images(args.level, args.strip, args.fix, args.interlace, paths);
}
