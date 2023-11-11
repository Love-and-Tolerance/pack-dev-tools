use image::{imageops, GenericImageView};
use pdtlib::pdtfs;
use std::env;
use std::path::MAIN_SEPARATOR as SLASH;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = args[1].to_string();
	let width = args[2].to_string().parse::<u32>().unwrap();
	let height = args[3].to_string().parse::<u32>().unwrap();
	unstitch_texture(filename, width, height);
}

fn unstitch_texture(filename: String, width: u32, height: u32) {
	let name = filename.split('.').collect::<Vec<&str>>()[0].to_string();
	let filetype = filename.split('.').collect::<Vec<&str>>()[1].to_string();
	let mut img = image::open(filename).unwrap();
	let image_width = img.dimensions().0;
	let image_height = img.dimensions().1;

	if image_width % width != 0 || image_height % height != 0 {
		panic!("Image dimensions not divisible by supplied tile dimension!");
	}

	let sprite_width = image_width / width;
	let sprite_height = image_height / height;

	let output_dir = format!(".{SLASH}output_dir");

	pdtfs::if_dir_exists_remove_and_remake_it(&output_dir);

	for y in 0..width as usize {
		for x in 0..height as usize {
			println!("{x}, {y}");
			let subimg = imageops::crop(
				&mut img,
				x as u32 * sprite_width,
				y as u32 * sprite_height,
				sprite_width,
				sprite_height,
			);
			subimg
				.to_image()
				.save(format!("{output_dir}{SLASH}{name}-{x}-{y}.{filetype}"))
				.unwrap();
		}
	}
}
