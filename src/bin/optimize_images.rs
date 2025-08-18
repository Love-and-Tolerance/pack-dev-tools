use camino::Utf8PathBuf;
use indoc::printdoc;
use oxipng::{InFile, Options, OutFile, optimize};
use pony::fs::find_files_in_dir;
use pony::threads::multithread;
use std::env::args;
use std::path::MAIN_SEPARATOR as SLASH;
use std::process::exit;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

fn main() -> Result<()> {
	let args: Vec<String> = args().skip(1).collect();
	parse_argument(&args)?;
	let dir = args.iter().next_back().unwrap();
	optimize_images(dir)?;
	Ok(())
}

fn optimize_images(dir: &str) -> Result<()> {
	let mut options = Options::from_preset(6);
	options.fix_errors = true;
	options.optimize_alpha = true;
	options.strip = oxipng::StripChunks::All;
	let recursive = true;
	let files = find_files_in_dir(dir, recursive)?;
	let images = files
		.into_iter()
		.filter(|file| file.ends_with("png"))
		.collect::<Vec<_>>();
	multithread(images, None, move |thread, image| {
		let input = InFile::Path(Utf8PathBuf::from(&image).into());
		let output = OutFile::from_path(Utf8PathBuf::from(&image).into());
		optimize(&input, &output, &options).unwrap();
		println!("Thread {thread:0>2} optimized image: {}", &image);
		None::<()>
	});
	Ok(())
}

fn parse_argument(args: &[String]) -> Result<()> {
	if args.is_empty() {
		return Err("No arguments provided!".into());
	}
	match args.first().unwrap().as_str() {
		"-h" | "--help" => {
			print_help();
			exit(0);
		}
		"-v" | "--version" => {
			println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
			exit(0);
		}
		_ => {}
	}
	Ok(())
}

fn print_help() {
	printdoc! {"
		{bin} {}

		Optimize .png files for resource packs.

		Usage Examples:
		  {bin} .{SLASH}resource-pack

		Options:
		  -h,  --help        Print help
		  -v,  --version     Print version\n",
		env!("CARGO_PKG_VERSION"),
		bin = env!("CARGO_BIN_NAME")
	}
}
