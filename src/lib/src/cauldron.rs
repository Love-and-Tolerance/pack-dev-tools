use super::{pdtcolor, pdtfs, pdtthread};
use colors_transform::{Color, Hsl, Rgb};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::sync::Arc;

pub fn cauldron(color: String, items: Vec<String>) {
	let output = pdtfs::create_output_dir("cauldron_output");
	pdtfs::copy_files_to_dir(output.clone(), items, false);
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	let files = pdtfs::find_files_in_dir(&output, recursive, &extensions);
	let color = pdtcolor::hex_to_hsl(color);
	dye_images_in_cauldron(files, color, 75.00);
}

pub fn dye_images_in_cauldron(images: Vec<String>, color: Hsl, saturation: f32) {
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
				image.split('/').last().unwrap()
			);
			let img =
				image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
			let (width, height) = img.dimensions();
			let mut new_image: RgbaImage =
				ImageBuffer::from_fn(width, height, |_, _| image::Rgba([0, 0, 0, 0]));
			for pixel in img.pixels() {
				let alpha = pixel.2 .0[3];
				let (x, y) = (pixel.0, pixel.1);
				let rgb = Rgb::from(pixel.2[0].into(), pixel.2[1].into(), pixel.2[2].into());
				let rgb = pdtcolor::rgb_to_hsl(rgb)
					.set_hue(color.get_hue())
					.set_saturation(*saturation)
					.to_rgb();
				let rgba = Rgba::from([
					rgb.get_red().round() as u8,
					rgb.get_green().round() as u8,
					rgb.get_blue().round() as u8,
					alpha,
				]);
				new_image.put_pixel(x, y, rgba);
			}
			new_image.save(&image).unwrap();

			None::<()>
		},
	);
}
