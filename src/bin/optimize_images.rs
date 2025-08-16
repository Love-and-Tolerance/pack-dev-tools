use camino::Utf8PathBuf;
use oxipng::{InFile, IndexSet, Options, OutFile, RowFilter, optimize};
use pony::fs::find_files_in_dir;
use std::{env::args, num::NonZero};

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

fn main() -> Result<()> {
	let args = args().collect::<Vec<_>>();
	optimize_images(&args[1])?;
	Ok(())
}

fn optimize_images(dir: &str) -> Result<()> {
	let mut options = Options::from_preset(6);
	options.deflate = oxipng::Deflaters::Zopfli {
		iterations: NonZero::new(255).unwrap(),
	};
	options.fix_errors = true;
	options.optimize_alpha = true;
	options.interlace = Some(oxipng::Interlacing::None);
	options.strip = oxipng::StripChunks::All;
	options.fast_evaluation = true;
	options.filter = IndexSet::from([
		RowFilter::Sub,
		RowFilter::Up,
		RowFilter::Average,
		RowFilter::Paeth,
		RowFilter::MinSum,
		RowFilter::Entropy,
		RowFilter::Bigrams,
		RowFilter::BigEnt,
		RowFilter::Brute,
	]);
	let recursive = true;
	let images = find_files_in_dir(dir, recursive)?;
	for image in images.iter().filter(|file| file.ends_with("png")) {
		println!("optimizing image: {}", &image);
		let input = InFile::Path(Utf8PathBuf::from(&image).into());
		let output = OutFile::from_path(Utf8PathBuf::from(&image).into());
		optimize(&input, &output, &options)?;
	Ok(())
	}
}
