use super::{pdtfs, pdtthread};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::MAIN_SEPARATOR as SLASH;
use std::sync::Arc;

pub fn infect_villagers(paths: Vec<String>, overlay: &[u8], resource_pack_conversion: bool) {
	let trigger_pixels: [(u32, u32); 6] = [(0, 0), (1, 0), (1, 1), (2, 0), (2, 1), (3, 0)];
	let overlay_pixels = image::load_from_memory(overlay)
		.unwrap_or_else(|_| panic!("Failed to load overlay image."))
		.pixels()
		.filter(|p| p.2 .0[3] == 255)
		.collect::<Vec<_>>();
	let extensions = Some(vec![".png".to_string()]);
	let texture_files = match resource_pack_conversion {
		true => resource_pack_conversion_setup(paths, extensions),
		false => {
			let output = pdtfs::create_output_dir("infected_ponies");
			pdtfs::copy_files_to_dir(output.clone(), paths, true);
			pdtfs::find_files_in_dir(&output, true, &extensions)
		}
	};
	villager_infector(texture_files, trigger_pixels, overlay_pixels);
}

pub fn resource_pack_conversion_setup(
	paths: Vec<String>, extensions: Option<Vec<String>>,
) -> Vec<String> {
	if paths.len() > 1 {
		panic!(
			"Expected 1 argument for resource pack conversion, found {}",
			paths.len()
		);
	}
	let location = format!(
		"Villager-Skin-Pack{s}assets{s}minelittlepony{s}textures{s}entity",
		s = SLASH
	);
	let pony_location = pdtfs::create_output_dir(&format!("{location}{SLASH}pony"));
	let zompony_location = pdtfs::create_output_dir(&format!("{location}{SLASH}zompony"));
	pdtfs::copy_dir_to_dir(&pony_location, paths[0].to_string(), true);
	for dir in pdtfs::find_dirs_in_dir(&pony_location, true).iter().rev() {
		let (location, folder) = dir.rsplit_once(SLASH).unwrap();
		let new_name = format!(
			"{location}{SLASH}{}",
			folder.replace(' ', "_").to_lowercase()
		);
		pdtfs::rename(dir, &new_name);
	}
	let remove_extensions = Some(vec![
		".md".to_string(),
		".txt".to_string(),
		".json".to_string(),
	]);
	pdtfs::delete_files_in_dir(&pony_location, true, &remove_extensions);
	pdtfs::if_dir_exists_remove_it(&format!("{}{SLASH}.git", pony_location));
	pdtfs::copy_dir_to_dir(&zompony_location, pony_location, true);
	pdtfs::find_files_in_dir(&zompony_location, true, &extensions)
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
				"[thread {:02} villager-infector] infecting pony: {}",
				thread_num,
				pony.rsplit(SLASH)
					.next()
					.unwrap()
					.trim_end_matches(".png")
					.replace('_', " ")
					.split_whitespace()
					.map(|s| format!("{}{}", &s[..1].to_uppercase(), &s[1..]))
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
			infected_pony
				.save(&pony)
				.unwrap_or_else(|_| panic!("Failed to save image: {pony}"));

			None::<()>
		},
	);
}
