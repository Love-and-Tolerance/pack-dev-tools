use clap::{value_parser, Parser};
use colors_transform::{Color, Hsl, Rgb};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use pdt::{pdtcolor, pdtfs, pdtthread};
use pdt::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Tint png images to a specified color and saturation.

example: .{s}cauldron F5B7D0 .{s}images
example: .{s}cauldron e6c343 -s 65 .{s}assets", s = SLASH),
	long_about = None)
]

struct Args {
	/// Hex color to tint images
	hex_color: String,
	#[arg(short,long,value_parser = value_parser!(u32).range(0..=100))]
	/// Saturation level [0..100]
	saturation: Option<u32>,
	/// List of files and folders to tint
	input_paths: Vec<String>,
}

fn main() {
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or_default()
		.extend_vec(args.input_paths);
	let saturation: Option<f32> = match args.saturation {
		Some(_) => Some(args.saturation.unwrap() as f32),
		None => None,
	};
	cauldron(args.hex_color, paths, saturation);
}

fn cauldron(color: String, items: Vec<String>, saturation: Option<f32>) {
	let output = pdtfs::create_output_dir("cauldron_output");
	pdtfs::copy_files_to_dir(output.clone(), items, false);
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	let files = pdtfs::find_files_in_dir(&output, recursive, &extensions);
	let color = pdtcolor::hex_to_hsl(color);
	dye_images_in_cauldron(files, color, saturation);
}

fn dye_images_in_cauldron(images: Vec<String>, color: Hsl, saturation: Option<f32>) {
	let color = Arc::new(color);
	let saturation = Arc::new(saturation);
	let images = images
		.into_iter()
		.map(|i| (i, Arc::clone(&color), Arc::clone(&saturation)))
		.collect();

	pdtthread::multithread(
		images,
		None,
		move |thread_num, (image, color, saturation)| {
			println!(
				"[thread {thread_num:02} cauldron] dying image: {}",
				image.split(SLASH).last().unwrap()
			);
			let img =
				image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
			let (width, height) = img.dimensions();
			let mut new_image: RgbaImage =
				ImageBuffer::from_fn(width, height, |_, _| image::Rgba([0, 0, 0, 0]));
			for pixel in img.pixels() {
				let a = pixel.2 .0[3];
				let (x, y) = (pixel.0, pixel.1);
				let hsl =
					Rgb::from(pixel.2[0].into(), pixel.2[1].into(), pixel.2[2].into()).to_hsl();
				let rgb = hsl
					.set_hue(color.get_hue())
					.set_saturation(saturation.unwrap_or(hsl.get_saturation()))
					.to_rgb()
					.as_tuple();
				let rgba = [rgb.0 as u8, rgb.1 as u8, rgb.2 as u8, a];
				new_image.put_pixel(x, y, Rgba(rgba));
			}
			new_image.save(&image).unwrap();

			None::<()>
		},
	);
}
