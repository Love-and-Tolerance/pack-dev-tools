use super::pdtfs;
use super::pdtthread;
use crate::json_format::{json_formatter, Indent, Json};
use crate::optimize_images::optimize_images;
use deltae::*;
use fs_extra::dir::{copy, CopyOptions};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::sync::{Arc, Mutex};
use std::{
    cmp::{Ordering, PartialOrd},
    path::MAIN_SEPARATOR as SLASH,
};

type Average = (LabValue, String);
type Distance = (f64, (u32, u32, Rgba<u8>), LabValue);

pub fn blockify(block: String, pack: String, optimize: bool) {
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
    if optimize {
        json_formatter(output.clone(), Json::Minify, Indent::Tab);
        optimize_images(output);
    }
}

fn get_average_colors(blocks: Vec<String>) -> Vec<Average> {
    let averages = Arc::new(Mutex::new(Vec::new()));

    let blocks = blocks
        .into_iter()
        .map(|b| (b, Arc::clone(&averages)))
        .collect();

    pdtthread::multithread(blocks, None, |thread_num, (image, averages)| {
        println!("[thread {thread_num} get_average_colors] averaging {image}");
        let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
        if img.dimensions().0 != 16 || img.dimensions().1 != 16 {
            return;
        }
        let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
        let mut distances: Vec<Distance> = vec![];
        for pixel in img.pixels() {
            let lab = get_lab(pixel);
            let mut distance: f64 = 0.0;
            for sub_pixel in img.pixels() {
                if sub_pixel.2 .0[3] < 255 {
                    return;
                }
                let sub_lab = get_lab(sub_pixel);
                let delta: f64 = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
                distance += delta;
            }
            distance /= pixel_count;
            distances.push((distance, pixel, lab));
        }
        distances.sort_by(|a, b| compare(&a.0, &b.0));
        if !distances.is_empty() {
            averages.lock().unwrap().push((distances[0].2, image));
        }
    });

    Arc::try_unwrap(averages).unwrap().into_inner().unwrap()
}

fn blockify_images(images: Vec<String>, blocks: Vec<Average>) {
    let pixels = Arc::new(Mutex::new(0u128));
    let blocks = Arc::new(blocks);
    let images = images
        .into_iter()
        .map(|i| (i, Arc::clone(&pixels), Arc::clone(&blocks)))
        .collect();

    pdtthread::multithread(images, None, |thread_num, (texture, pixels, blocks)| {
        let p = pixels.lock().unwrap();
        println!(
            "[thread {thread_num} blockify_images] [{} output pixels] starting {texture}",
            *p
        );
        drop(p);

        let img =
            image::open(&texture).unwrap_or_else(|_| panic!("Failed to load image: {texture}"));
        let (width, height) = img.dimensions();
        let mut new_texture: RgbaImage =
            ImageBuffer::from_fn(width * 16, height * 16, |_, _| image::Rgba([0, 0, 0, 0]));
        for pixel in img.pixels() {
            let alpha = pixel.2 .0[3];
            if alpha == 0 {
                continue;
            }
            let (x, y) = (pixel.0, pixel.1);
            let mut distances: Vec<(f64, String)> = vec![];
            let lab = get_lab(pixel);
            for block in blocks.iter() {
                let delta: f64 = DeltaE::new(lab, block.0, DE2000).value().to_owned().into();
                distances.push((delta, block.1.to_owned()));
            }
            let selected = get_closest_match(lab, distances, 1, texture.clone());
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

        new_texture.save(&texture).unwrap();

        let mut p = pixels.lock().unwrap();
        *p += u128::from((width * 16) * (height * 16));
        drop(p);
    });
}

fn get_closest_match(lab: LabValue, mut distances: Vec<(f64, String)>, index: usize, texture: String) -> String {
    distances.sort_by(|a, b| compare(&a.0, &b.0));
    let matches = distances
        .iter()
        .filter(|item| item.0 == distances[0].0)
        .collect::<Vec<&(f64, String)>>();
    if matches.len() == 1 {
        println!("{index} - Found a block for a pixel in {}", texture);
        matches[0].1.to_owned()
    } else {
        let mut averages: Vec<(LabValue, String)> = vec![];
        for (_, image) in matches.iter() {
            let img =
                image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
            let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
            let mut sub_distances: Vec<Distance> = vec![];
            for pixel in img.pixels() {
                let lab = get_lab(pixel);
                let mut distance: f64 = 0.0;
                for sub_pixel in img.pixels() {
                    let sub_lab = get_lab(sub_pixel);
                    let delta: f64 = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
                    distance += delta;
                }
                distance /= pixel_count;
                sub_distances.push((distance, pixel, lab));
            }
            sub_distances.sort_by(|a, b| compare(&a.0, &b.0));
            sub_distances.dedup_by_key(|a| a.0);
            if index < sub_distances.len() {
                averages.push((sub_distances[index].2, image.to_string()));
            }
        }
        let mut new_distances = vec![];
        for block in averages.iter() {
            let delta: f64 = DeltaE::new(lab, block.0, DE2000).value().to_owned().into();
            new_distances.push((delta, block.1.to_owned()));
        }
        if index < 5 {
            println!("trying again to find a block for a pixel in {}", texture);
            get_closest_match(lab, new_distances, index + 1, texture)
        } else {
            new_distances.sort_by_key(|k| k.1.clone());
            println!("Found a block {} for a pixel in {} by brute force", new_distances[0].1, texture);
            new_distances[0].1.to_owned()
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

fn compare<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
