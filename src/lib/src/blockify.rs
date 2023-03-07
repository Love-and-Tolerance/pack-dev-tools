use super::pdtfs;
use deltae::*;
use image::{imageops, GenericImageView};
use lab::Lab;
use std::path::MAIN_SEPARATOR as SLASH;

pub fn blockify(block: String, pack: String) {
    pdtfs::check_if_dir_exists(&block);
    pdtfs::check_if_dir_exists(&pack);
    let extensions = Some(vec![".png"]);
    let block_files = pdtfs::find_files_in_dir(&block, false, &extensions);
    let texture_files = pdtfs::find_files_in_dir(&pack, true, &extensions);
    let average_block_colours = get_average_colors(block_files);
}

fn get_average_colors(blocks: Vec<String>) {
    let mut averages: Vec<(i8, String)>;
    for image in blocks {
        let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
        if img.dimensions().0 != img.dimensions().1 {
            continue;
        }
        for pixel in img.pixels() {
            let rgb = [[pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]];
            let lab = lab::rgbs_to_labs(&rgb)[0];
            let lab = LabValue {
                l: lab.l,
                a: lab.a,
                b: lab.b,
            };
            for sub_pixel in img.pixels() {
                let sub_rgb = [[sub_pixel.2 .0[0], sub_pixel.2 .0[1], sub_pixel.2 .0[2]]];
                let sub_lab = lab::rgbs_to_labs(&sub_rgb)[0];
                let sub_lab = LabValue {
                    l: sub_lab.l,
                    a: sub_lab.a,
                    b: sub_lab.b,
                };
                let de0 = DeltaE::new(&lab, &sub_lab, DE2000);
                let de1 = lab.delta(&sub_lab, DE2000);
                assert_eq!(de0, de1);
                println!("{de0} {de1} {pixel:?} {sub_pixel:?} {image}");
            }
        }
    }
}
