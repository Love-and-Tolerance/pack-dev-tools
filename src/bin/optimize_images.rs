use camino::Utf8PathBuf;
use oxipng::{InFile, Options, OutFile, optimize};
use pony::fs::find_files_in_dir;
use pony::threads::multithread;
use std::env::args;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

fn main() -> Result<()> {
	let dir = args().next_back().unwrap();
	optimize_images(&dir)?;
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
