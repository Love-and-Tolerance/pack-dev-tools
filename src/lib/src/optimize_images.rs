use super::pdtfs::get_files_in_list;
use clap::ValueEnum;
use oxipng::{optimize, InFile, Options, OutFile};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
pub enum Strip {
	None,
	Safe,
	All,
}

pub fn optimize_images(level: u8, strip: Strip, fix: bool, interlace: bool, paths: Vec<String>) {
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
	let images = get_files_in_list(paths, recursive, extensions, &EXCLUDE_DIR_NAME);
	for image in images {
		println!("optimizing image: {}", &image);
		let input = InFile::Path(PathBuf::from(&image));
		let output = OutFile::Path(Some(PathBuf::from(&image)));
		optimize(&input, &output, &options).expect("Failed to optimize image.");
	}
}
