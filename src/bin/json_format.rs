use camino::Utf8Path;
use indoc::printdoc;
use pony::fs::find_files_in_dir;
use pony::json::{JsonFormat, format_json};
use pony::regex::matches;
use regex::Regex;
use std::process::exit;
use std::{env, fs};

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

fn main() -> Result<()> {
	let args: Vec<String> = env::args().skip(1).collect();
	let format = parse_argument(&args)?;
	let dir = args.last().unwrap();
	if !Utf8Path::new(dir).is_dir() {
		return Err("Last argument is not a directory!".into());
	}
	let includes = &Some(Regex::new(r".*\.(json|mcmeta)$")?);
	let files = find_files_in_dir(dir, true)?;
	let files = files
		.iter()
		.filter(|file| matches(file, includes, &None))
		.collect::<Vec<_>>();
	for file in files {
		let json = fs::read_to_string(file)?;
		let text = format_json(&json, &format)?;
		fs::write(file, text.as_bytes())?;
	}
	Ok(())
}

fn parse_argument(args: &[String]) -> Result<JsonFormat> {
	if args.is_empty() {
		return Err("No arguments provided!".into());
	}
	match args.iter().next().unwrap().as_str() {
		"-m" | "--minify" => Ok(JsonFormat::Minify),
		"-t" | "--tab" => Ok(JsonFormat::Tab),
		"-s" | "--space" => match args.iter().next() {
			None => Err("No space count provided!".into()),
			Some(count) => {
				let count = count.parse::<u8>();
				match count {
					Ok(count) => Ok(JsonFormat::Space(count)),
					Err(_) => Err("Failed to parse space count".into()),
				}
			}
		},
		"-h" | "--help" => {
			print_help();
			exit(0);
		}
		"-v" | "--version" => {
			println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
			exit(0);
		}
		_ => Ok(JsonFormat::Tab),
	}
}

fn print_help() {
	printdoc! {"
		{} {}

		Format .json and .mcmeta files with tabs or spaces.

		Usage Examples:
		  json_format ./dir
		  json_format -s 3 ./dir

		Options:
		  -m,  --minify      Format onto one line
		  -s,  --space       Format with spaces (1-8)
		  -t,  --tab         Format with tabs (default)
		  -h,  --help        Print help
		  -v,  --version     Print version\n",
		env!("CARGO_BIN_NAME"),
		env!("CARGO_PKG_VERSION")
	}
}
