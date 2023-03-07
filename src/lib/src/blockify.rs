use super::pdtfs;
use deltae::*;
use image::{imageops, GenericImageView, Rgba};
use lab::Lab;
use std::{cmp::Ordering, path::MAIN_SEPARATOR as SLASH};

type Distance = (f64, (u32, u32, Rgba<u8>));

pub fn blockify(block: String, pack: String) {
    pdtfs::check_if_dir_exists(&block);
    pdtfs::check_if_dir_exists(&pack);
    let extensions = Some(vec![".png"]);
    let block_files = pdtfs::find_files_in_dir(&block, false, &extensions);
    let texture_files = pdtfs::find_files_in_dir(&pack, true, &extensions);
    let average_block_colours = get_average_colors(block_files);
}

fn get_average_colors(blocks: Vec<String>) {
    let mut averages: Vec<(Rgba<u8>, String)>;
    for image in blocks {
        let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
        if img.dimensions().0 != img.dimensions().1 {
            continue;
        }
        let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
        let mut distances: Vec<Distance> = vec![];
        let mut delta: f64 = 0.0;
        for pixel in img.pixels() {
            if pixel.2 .0[3] != 255 {
                continue;
            }
            let lab = get_lab(pixel);
            let mut distance: f64 = 0.0;
            for sub_pixel in img.pixels() {
                let sub_lab = get_lab(sub_pixel);
                delta = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
                distance += delta;
            }
            distance /= pixel_count;
            distances.push((distance, pixel));
        }
        distances.sort_by(compare_distances);
        if distances.len() > 0 {
            println!("{:?}, {}, {}", distances[0], distances.len(), image);
        }
    }
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
