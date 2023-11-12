use clap::{value_parser, Parser};
use deltae::*;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use pdt::{pdtcolor, pdtfs, pdtthread, pdttrait};
use pdt::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;
use std::sync::{Arc, Mutex};

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Blockify images by turning every pixel into a block texture.

example: .{s}blockify 16 .{s}assets{s}minecraft{s}textures{s}blocks .{s}assets", s = SLASH),
	long_about = None)
]

struct Args {
	#[arg(value_parser = value_parser!(u32).range(2..=32))]
	/// The width or height of the block textures [2..32]
	block_pixels: u32,
	/// Path to block textures
	blocks_path: String,
	/// List of files and folders to blockify
	input_paths: Vec<String>,
}

type Pixel = (f64, Rgba<u8>, LabValue);
type Block = (String, Vec<Pixel>);

fn main() {
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or_default()
		.extend_vec(args.input_paths);
	blockify(args.block_pixels, args.blocks_path, paths);
}

fn blockify(pixels: u32, blocks_dir: String, paths: Vec<String>) {
	let output = pdtfs::create_output_dir("blockify_output");
	pdtfs::copy_files_to_dir(output.clone(), paths, true);
	let extensions = Some(vec![".png".to_string()]);
	let block_files = pdtfs::find_files_in_dir(&blocks_dir, false, &extensions);
	let texture_files = pdtfs::find_files_in_dir(&output, true, &extensions);
	let average_block_colors: Vec<Block> = get_average_colors(block_files, pixels);
	blockify_images(texture_files, average_block_colors, pixels);
}

fn get_average_colors(blocks: Vec<String>, pixels: u32) -> Vec<Block> {
	pdtthread::multithread(blocks, None, move |thread_num, image| {
		println!(
			"[thread {thread_num:02} get_average_colors] averaging {}",
			image.split(SLASH).last().unwrap()
		);

		let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
		if img.dimensions().0 != pixels || img.dimensions().1 != pixels {
			return None;
		}

		let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
		let mut distances: Vec<Pixel> = vec![];

		for pixel in img.pixels() {
			let lab = pdtcolor::rgb_to_lab(pixel);
			let mut distance: f64 = 0.0;
			for sub_pixel in img.pixels() {
				if sub_pixel.2 .0[3] < 255 {
					return None;
				}
				let sub_lab = pdtcolor::rgb_to_lab(sub_pixel);
				let delta: f64 = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
				distance += delta;
			}
			distance /= pixel_count;
			distances.push((distance, pixel.2, lab));
		}

		distances.sort_by(|a, b| pdttrait::compare(&a.0, &b.0));
		distances.dedup();

		if distances.is_empty() {
			None
		} else {
			Some((image, distances))
		}
	})
}

fn blockify_images(images: Vec<String>, blocks: Vec<Block>, block_pixels: u32) {
	let pixels = Arc::new(Mutex::new(0u128));
	let blocks = Arc::new(blocks);
	let images = images
		.into_iter()
		.map(|i| (i, Arc::clone(&pixels), Arc::clone(&blocks)))
		.collect();

	pdtthread::multithread(
		images,
		None,
		move |thread_num, (texture, pixels, blocks)| {
			let p = pixels.lock().unwrap();
			println!(
				"[thread {thread_num:02} blockify_images] [{:010} output pixels] starting {}",
				*p,
				texture.split(SLASH).last().unwrap()
			);
			drop(p);

			let img =
				image::open(&texture).unwrap_or_else(|_| panic!("Failed to load image: {texture}"));
			let (width, height) = img.dimensions();
			let mut new_texture: RgbaImage =
				ImageBuffer::from_fn(width * block_pixels, height * block_pixels, |_, _| {
					image::Rgba([0, 0, 0, 0])
				});

			for pixel in img.pixels() {
				let a = pixel.2 .0[3];
				if a == 0 {
					continue;
				}
				let (x, y) = (pixel.0, pixel.1);
				let lab = pdtcolor::rgb_to_lab(pixel);
				let selected = get_closest_match(lab, blocks.to_vec());
				let block_img = image::open(&selected)
					.unwrap_or_else(|_| panic!("Failed to load image: {selected}"));
				for sub_pixel in block_img.pixels() {
					let sub_x = (x * block_pixels) + sub_pixel.0;
					let sub_y = (y * block_pixels) + sub_pixel.1;
					let rgba = [sub_pixel.2 .0[0], sub_pixel.2 .0[1], sub_pixel.2 .0[2], a];
					new_texture.put_pixel(sub_x, sub_y, image::Rgba(rgba));
				}
			}

			new_texture.save(&texture).unwrap();

			let mut p = pixels.lock().unwrap();
			*p += u128::from((width * block_pixels) * (height * block_pixels));
			drop(p);

			None::<()>
		},
	);
}

fn get_closest_match(lab: LabValue, blocks: Vec<Block>) -> String {
	let mut new_blocks = blocks
		.into_iter()
		.map(|block| {
			let delta = *DeltaE::new(lab, block.1[0].2, DE2000).value() as f64;
			(delta, block)
		})
		.collect::<Vec<_>>();
	new_blocks.sort_by(|a, b| pdttrait::compare(&a.0, &b.0));

	let matches = new_blocks
		.iter()
		.filter(|item| item.0 == new_blocks[0].0)
		.collect::<Vec<_>>();

	if matches.len() == 1 {
		return matches[0].1 .0.clone();
	}
	let next_blocks = matches
		.iter()
		.filter(|block| block.1 .1.len() > 1)
		.map(|block| (block.1 .0.to_string(), block.1 .1[1..].to_vec()))
		.collect::<Vec<_>>();
	if next_blocks.len() > 1 {
		return get_closest_match(lab, next_blocks);
	}
	matches[0].1 .0.clone()
}
