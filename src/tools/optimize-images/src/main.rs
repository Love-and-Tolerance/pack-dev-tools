use camino::Utf8PathBuf;
use clap::{value_parser, Parser, ValueEnum};
use pdtlib::optimize_images::optimize_images;
use pdtlib::pdtstdin;

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
	paths: Vec<Utf8PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Strip {
	None,
	Safe,
	All,
}

fn main() {
	let args = Args::parse();
	println!("{args:?}");
	let stdin = pdtstdin::get_stdin().unwrap_or(["test".to_string()].to_vec());

	for thing in stdin {
		println!("{thing}");
	}
	//optimize_images(dir);
}
