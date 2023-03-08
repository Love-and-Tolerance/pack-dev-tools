use super::pdtfs;
use deltae::*;
use fs_extra::dir::{CopyOptions, copy};
use image::{imageops, GenericImageView, Rgba};
use lab;
use std::{cmp::Ordering, path::MAIN_SEPARATOR as SLASH};

type Average = (LabValue, String);
type Distance = (f64, (u32, u32, Rgba<u8>), LabValue);

pub fn blockify(block: String, pack: String) {
    pdtfs::check_if_dir_exists(&block);
    pdtfs::check_if_dir_exists(&pack);
    let output = format!(".{SLASH}output");
    pdtfs::if_dir_exists_remove_and_remake_it(&output);
    let mut options = CopyOptions::new();
    options.content_only = true;
    copy(pack, &output, &options)
        .unwrap_or_else(|_| panic!("Failed to copy old release to {} directory.", &output));
    let extensions = Some(vec![".png"]);
    let block_files = pdtfs::find_files_in_dir(&block, false, &extensions);
    let texture_files = pdtfs::find_files_in_dir(&output, true, &extensions);
    let average_block_colors = get_average_colors(block_files);
    blockify_images(texture_files, average_block_colors);
}

fn get_average_colors(blocks: Vec<String>) -> Vec<Average> {
    let mut averages: Vec<Average> = vec![];
    'block: for image in blocks {
        let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
        if img.dimensions().0 != img.dimensions().1 {
            continue;
        }
        let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
        let mut distances: Vec<Distance> = vec![];
        for pixel in img.pixels() {
            let lab = get_lab(pixel);
            let mut distance: f64 = 0.0;
            for sub_pixel in img.pixels() {
                if sub_pixel.2 .0[3] < 255 {
                    continue 'block;
                }
                let sub_lab = get_lab(sub_pixel);
                let delta: f64 = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
                distance += delta;
            }
            distance /= pixel_count;
            distances.push((distance, pixel, lab));
        }
        distances.sort_by(compare_distances);
        if distances.len() > 0 {
            averages.push((distances[0].2, image));
        }
    }
    averages
}

fn get_lab(pixel: (u32, u32, Rgba<u8>)) -> LabValue {
    let rgb = [[pixel.2 .0[0], pixel.2 .0[1], pixel.2 .0[2]]];
    let lab = lab::rgbs_to_labs(&rgb)[0];
    LabValue {
        l: lab.l,
        a: lab.a,
        b: lab.b,
    }
}

fn compare_distances(a: &Distance, b: &Distance) -> Ordering {
    if a.0 < b.0 {
        return Ordering::Less;
    } else if a.0 > b.0 {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn blockify_images(images: Vec<String>, blocks: Vec<Average>) {

}