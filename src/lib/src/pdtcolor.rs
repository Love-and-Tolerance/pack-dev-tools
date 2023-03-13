use colors_transform::Rgb;
use deltae::LabValue;
use image::Rgba;

pub fn rgb_to_lab(pixel: (u32, u32, Rgba<u8>)) -> LabValue {
	let rgb = [[pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]];
	let lab = lab::rgbs_to_labs(&rgb)[0];
	LabValue {
		l: lab.l,
		a: lab.a,
		b: lab.b,
	}
}

pub fn hex_to_rgb(hex: String) -> Rgb {
	let hex = if hex.starts_with('#') {
		hex
	} else {
		format!("#{hex}")
	};
	Rgb::from_hex_str(&hex).unwrap()
}
