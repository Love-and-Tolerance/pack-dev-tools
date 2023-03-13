use deltae::LabValue;
use image::Rgba;
use std::collections::HashMap;

pub fn rgb_to_lab(pixel: (u32, u32, Rgba<u8>)) -> LabValue {
	let rgb = [[pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]];
	let lab = lab::rgbs_to_labs(&rgb)[0];
	LabValue {
		l: lab.l,
		a: lab.a,
		b: lab.b,
	}
}

pub fn get_hex_from_map(hex: &char) -> u8 {
	let map = HashMap::from([
		('a', 11),
		('b', 12),
		('c', 13),
		('d', 14),
		('e', 15),
		('f', 16),
	]);
	*map.get(&hex).unwrap()
}

pub fn hex_to_rgb(hex: String) -> (u8, u8, u8) {
	let rgb = hex
		.chars()
		.into_iter()
		.filter(|c| c != &'#')
		.map(|c| convert_hex_char(c))
		.collect::<Vec<_>>();

	let r = rgb[0] * rgb[0] + rgb[1] - 1;
	let g = rgb[2] * rgb[2] + rgb[3] - 1;
	let b = rgb[4] * rgb[4] + rgb[5] - 1;
	(r, g, b)
}

pub fn convert_hex_char(hex: char) -> u8 {
	TryInto::<u8>::try_into(hex).unwrap_or(get_hex_from_map(&hex))
}
