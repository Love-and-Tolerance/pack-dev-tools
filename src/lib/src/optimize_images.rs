use super::pdtfs::{check_if_dir_exists, find_files_in_dir};
use oxipng::{optimize, InFile, Options, OutFile};
use std::path::PathBuf;

pub fn optimize_images(dir: String) {
	check_if_dir_exists(&dir);
	let recursive = true;
	let extensions = Some(vec![".png"]);
	let images = find_files_in_dir(&dir, recursive, &extensions);
	let mut options = Options::from_preset(6);
	options.fix_errors = true;
	options.interlace = Some(oxipng::Interlacing::Adam7);
	options.strip = oxipng::Headers::Safe;
	for image in images {
		println!("optimizing image: {}", &image);
		let input = InFile::Path(PathBuf::from(&image));
		let output = OutFile::Path(Some(PathBuf::from(&image)));
		optimize(&input, &output, &options).expect("Failed to optimize image.");
	}
}
