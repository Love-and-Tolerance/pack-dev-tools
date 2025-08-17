use camino::Utf8PathBuf;
use oxipng::{InFile, Options, OutFile, optimize};
use pony::{fs::find_files_in_dir, threads::multithread};
use std::env::args;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

fn main() -> Result<()> {
	let args = args().collect::<Vec<_>>();
	optimize_images(&args[1])?;
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
	multithread(images, None, move |_, image| {
		println!("optimizing image: {}", &image);
		let input = InFile::Path(Utf8PathBuf::from(&image).into());
		let output = OutFile::from_path(Utf8PathBuf::from(&image).into());
		optimize(&input, &output, &options).unwrap();
		None::<()>
	});
	Ok(())
}

#[cfg(test)]
mod tests {
	use camino::Utf8Path;
	use pony::bytes::{FormatType, format_size_bytes};

	use super::*;
	use std::fs;

	#[test]
	fn test_compression() -> Result<()> {
		if Utf8Path::new("/home/velvetremedy/oitd/out").is_dir() {
			fs::remove_dir_all("/home/velvetremedy/oitd/out")?;
		}
		fs::create_dir_all("/home/velvetremedy/oitd/out")?;
		let files = find_files_in_dir("/home/velvetremedy/oitd/base", true)?;
		for file in &files {
			fs::copy(file, file.replace("/base/", "/out/"))?;
		}
		optimize_images("/home/velvetremedy/oitd/out")?;
		let base_size = format_size_bytes(count_size(&files)? as f64, FormatType::Abbreviation)?;
		println!("Base files: {base_size}");
		let files = find_files_in_dir("/home/velvetremedy/oitd/out", true)?;
		let new_size = format_size_bytes(count_size(&files)? as f64, FormatType::Abbreviation)?;
		println!("Compressed files: {new_size}");
		Ok(())
	}

	fn count_size(files: &[String]) -> Result<usize> {
		let bytes = files
			.iter()
			.filter(|file| file.ends_with(".png"))
			.map(|file| {
				Ok::<_, Box<dyn std::error::Error>>(fs::File::open(file)?.metadata()?.len() as usize)
			})
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.sum();
		Ok(bytes)
	}
}
