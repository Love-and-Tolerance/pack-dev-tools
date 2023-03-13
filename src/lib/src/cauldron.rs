use super::pdtfs;
use image::Rgb;

pub fn cauldron(color: String, items: Vec<String>) {
	let output = pdtfs::create_output_dir("cauldron_output");
	pdtfs::copy_files_to_dir(output.clone(), items, false);
	let recursive = true;
	let extensions = Some(vec![".png".to_string()]);
	let files = pdtfs::find_files_in_dir(&output, recursive, &extensions);
}

pub fn change_images_hue(color: Rgb<[u8; 3]>, images: Vec<String>) {}
