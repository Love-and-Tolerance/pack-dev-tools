use super::pdtfs;
use deltae::*;
use fs_extra::dir::{copy, CopyOptions};
use image::{imageops, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use lab;
use rand::seq::SliceRandom;
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
        if img.dimensions().0 != 16 && img.dimensions().1 != 16 {
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
    } else {
        return Ordering::Equal;
    }
}

fn blockify_images(images: Vec<String>, blocks: Vec<Average>) {
    for texture in images {
        println!("{}", &texture);
        let img =
            image::open(&texture).unwrap_or_else(|_| panic!("Failed to load image: {texture}"));
        let (width, height) = img.dimensions();
        let mut new_texture: RgbaImage =
            ImageBuffer::from_fn(width * 16, height * 16, |_, _| image::Rgba([0, 0, 0, 0]));
        for pixel in img.pixels() {
            if pixel.2 .0[3] == 0 {
                continue;
            }
            let (x, y) = (pixel.0, pixel.1);
            let alpha = pixel.2 .0[3];
            let mut distances: Vec<(f64, String)> = vec![];
            let lab = get_lab(pixel);
            for block in &blocks {
                let delta: f64 = DeltaE::new(lab, block.0, DE2000).value().to_owned().into();
                distances.push((delta, block.1.to_owned()));
            }
            distances.sort_by(compare_block_distances);
            let matches = distances
                .iter()
                .filter(|item| item.0 == distances[0].0)
                .collect::<Vec<&(f64, String)>>();
            let mut selected: String;
            if matches.len() == 1 {
                selected = matches[0].1.to_owned();
            } else {
                selected = matches
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .1
                    .to_owned();
            }
            let block_img = image::open(&selected)
                .unwrap_or_else(|_| panic!("Failed to load image: {selected}"));
            for sub_pixel in block_img.pixels() {
                let sub_x = (x * 16) + sub_pixel.0;
                let sub_y = (y * 16) + sub_pixel.1;
                let rgba = Rgba::from([
                    sub_pixel.2 .0[0],
                    sub_pixel.2 .0[1],
                    sub_pixel.2 .0[2],
                    alpha,
                ]);
                new_texture.put_pixel(sub_x, sub_y, rgba);
            }
        }
        new_texture.save(texture).unwrap();
    }
}

fn compare_block_distances(a: &(f64, String), b: &(f64, String)) -> Ordering {
    if a.0 < b.0 {
        return Ordering::Less;
    } else if a.0 > b.0 {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}
