use camino::Utf8PathBuf;
use clap::ValueEnum;
use clap::{value_parser, Parser};
use oxipng::{optimize, InFile, Options, OutFile};
use pdt::pdtfs::get_files_in_list;
use pdt::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
	bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Optimize all png files in given path(s).

example: .{s}optimize-images -l4 a.png .{s}assets{s}
example: .{s}optimize-images --strip safe a.png b.png", s = SLASH),
	long_about = None)]

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
		.unwrap_or_default()
		.extend_vec(args.paths);
	optimize_images(args.level, args.strip, args.fix, args.interlace, paths);
}

#[derive(Clone, Debug, ValueEnum)]
enum Strip {
	None,
	Safe,
	All,
}

fn optimize_images(level: u8, strip: Strip, fix: bool, interlace: bool, paths: Vec<String>) {
	let mut options = Options::from_preset(level);
	options.strip = match strip {
		Strip::None => oxipng::Headers::None,
		Strip::Safe => oxipng::Headers::Safe,
		Strip::All => oxipng::Headers::All,
	};
	options.fix_errors = fix;
	options.interlace = match interlace {
		true => Some(oxipng::Interlacing::Adam7),
		false => None,
	};
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	const EXCLUDE_DIR_NAME: bool = false;
	let images = get_files_in_list(paths, recursive, extensions, EXCLUDE_DIR_NAME, true);
	for image in images {
		println!("optimizing image: {}", &image);
		let input = InFile::Path(Utf8PathBuf::from(&image).into());
		let output = OutFile::Path(Some(Utf8PathBuf::from(&image).into()));
		optimize(&input, &output, &options).expect("Failed to optimize image.");
	}
}
