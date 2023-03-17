use super::{pdtcolor, pdtfs, pdtthread, pdttrait};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::sync::{Arc, Mutex};

pub fn infect_villagers(paths: Vec<String>, overlay: &[u8]) {
	let overlay = image::load_from_memory(overlay)
		.unwrap_or_else(|_| panic!("Failed to load overlay image."));
	for pixel in overlay.pixels() {
		if pixel.2 .0[3] < 255 {
			continue;
		}
		println!("{:?}", pixel);
	}
}
