use super::{pdtfs, pdtthread};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::sync::Arc;

pub fn infect_villagers(paths: Vec<String>, overlay: &[u8]) {
	let trigger_pixels: [(u32, u32); 6] = [(0, 0), (1, 0), (1, 1), (2, 0), (2, 1), (3, 0)];
	let overlay_pixels = image::load_from_memory(overlay)
		.unwrap_or_else(|_| panic!("Failed to load overlay image."))
		.pixels()
		.filter(|p| p.2 .0[3] == 255)
		.collect::<Vec<_>>();
	let output = pdtfs::create_output_dir("infected_ponies");
	pdtfs::copy_files_to_dir(output.clone(), paths, true);
	let extensions = Some(vec![".png".to_string()]);
	let texture_files = pdtfs::find_files_in_dir(&output, true, &extensions);
	villager_infector(texture_files, trigger_pixels, overlay_pixels);
}

pub fn villager_infector(
	ponies: Vec<String>, trigger_pixels: [(u32, u32); 6], overlay_pixels: Vec<(u32, u32, Rgba<u8>)>,
) {
	let trigger_pixels = Arc::new(trigger_pixels);
	let overlay_pixels = Arc::new(overlay_pixels);
	let ponies = ponies
		.into_iter()
		.map(|p| (p, Arc::clone(&trigger_pixels), Arc::clone(&overlay_pixels)))
		.collect();

	pdtthread::multithread(
		ponies,
		None,
		move |thread_num, (pony, trigger_pixels, overlay_pixels)| {
			println!(
				"[thread {thread_num:02} villager-infector] infecting pony: {}",
				pony.split('/')
					.last()
					.unwrap()
					.split('.')
					.collect::<Vec<_>>()[0]
					.split('_')
					.collect::<Vec<_>>()
					.iter()
					.map(|w| format!("{}{}", &w[..0].to_uppercase(), &w[1..]))
					.collect::<Vec<_>>()
					.join(" ")
			);
			let img = image::open(&pony).unwrap_or_else(|_| panic!("Failed to load image: {pony}"));
			let (width, height) = img.dimensions();
			let mut infected_pony: RgbaImage =
				ImageBuffer::from_fn(width, height, |_, _| image::Rgba([0, 0, 0, 0]));
			for pixel in img.pixels() {
				let a = pixel.2 .0[3];
				if a == 0 {
					continue;
				}
				let (x, y) = (pixel.0, pixel.1);
				let average =
					(pixel.2 .0[0] as f32 + pixel.2 .0[1] as f32 + pixel.2 .0[2] as f32) / 3.0;
				let new_r = (average + ((pixel.2 .0[0] as f32 - average) / 2.0)).round() as u8;
				let new_g = (average + ((pixel.2 .0[1] as f32 - average) / 2.0)).round() as u8;
				let new_b = (average + ((pixel.2 .0[2] as f32 - average) / 2.0)).round() as u8;
				let rgba = [new_r, new_g, new_b, a];
				infected_pony.put_pixel(x, y, image::Rgba(rgba));
			}
			for trigger_pixel in trigger_pixels.iter() {
				infected_pony.put_pixel(
					trigger_pixel.0,
					trigger_pixel.1,
					img.get_pixel(trigger_pixel.0, trigger_pixel.1),
				);
			}
			for overlay_pixel in overlay_pixels.iter() {
				infected_pony.put_pixel(overlay_pixel.0, overlay_pixel.1, overlay_pixel.2);
			}
			infected_pony.save(&pony).unwrap();

			None::<()>
		},
	);
}
