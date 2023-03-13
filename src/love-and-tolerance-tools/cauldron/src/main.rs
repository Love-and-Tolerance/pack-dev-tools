use pdtlib::cauldron::cauldron;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let color = args
		.get(1)
		.expect("first arg should be the color you want to tint the images.")
		.to_owned();
	let image_locations = args
		.get(2..)
		.expect("all args after should be file or folder locations of the images you want to tint.")
		.to_owned();
	cauldron(color, image_locations);
}
