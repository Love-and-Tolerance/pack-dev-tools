use super::json_format::{json_formatter, Indent, Json};
use super::optimize_images::optimize_images;
use super::pdtfs;
use super::pdtthread;
use deltae::*;
use fs_extra::dir::{copy, CopyOptions};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::sync::{Arc, Mutex};
use std::{
    cmp::{Ordering, PartialOrd},
    path::MAIN_SEPARATOR as SLASH,
};

type Pixel = (f64, Rgba<u8>, LabValue);
type Block = (String, Vec<Pixel>);

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
    let average_block_colors: Vec<Block> = get_average_colors(block_files);
    blockify_images(texture_files, average_block_colors);
    if optimize {
        json_formatter(output.clone(), Json::Minify, Indent::Tab);
        optimize_images(output);
    }
}

fn get_average_colors(blocks: Vec<String>) -> Vec<Block> {
    pdtthread::multithread(blocks, None, |thread_num, image| {
        println!(
            "[thread {thread_num:02} get_average_colors] averaging {}",
            image.split('/').last().unwrap()
        );

        let img = image::open(&image).unwrap_or_else(|_| panic!("Failed to load image: {image}"));
        if img.dimensions().0 != 16 || img.dimensions().1 != 16 {
            return None;
        }

        let pixel_count: f64 = (img.dimensions().0 * img.dimensions().1).into();
        let mut distances: Vec<Pixel> = vec![];

        for pixel in img.pixels() {
            let lab = get_lab(pixel);
            let mut distance: f64 = 0.0;
            for sub_pixel in img.pixels() {
                if sub_pixel.2 .0[3] < 255 {
                    return None;
                }
                let sub_lab = get_lab(sub_pixel);
                let delta: f64 = DeltaE::new(lab, sub_lab, DE2000).value().to_owned().into();
                distance += delta;
            }
            distance /= pixel_count;
            distances.push((distance, pixel.2, lab));
        }

        distances.sort_by(|a, b| compare(&a.0, &b.0));
        distances.dedup();

        if distances.is_empty() {
            None
        } else {
            Some((image, distances))
        }
    })
}

fn blockify_images(images: Vec<String>, blocks: Vec<Block>) {
    let pixels = Arc::new(Mutex::new(0u128));
    let blocks = Arc::new(blocks);
    let images = images
        .into_iter()
        .map(|i| (i, Arc::clone(&pixels), Arc::clone(&blocks)))
        .collect();

    pdtthread::multithread(images, None, |thread_num, (texture, pixels, blocks)| {
        let p = pixels.lock().unwrap();
        println!(
            "[thread {thread_num:02} blockify_images] [{:010} output pixels] starting {}",
            *p,
            texture.split('/').last().unwrap()
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
            let lab = get_lab(pixel);
            let selected = get_closest_match(lab, blocks.to_vec());
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

        None::<()>
    });
}

fn get_closest_match(lab: LabValue, blocks: Vec<Block>) -> String {
    let mut new_blocks = blocks
        .into_iter()
        .map(|block| {
            let delta = *DeltaE::new(lab, block.1[0].2, DE2000).value() as f64;
            (delta, block)
        })
        .collect::<Vec<_>>();
    new_blocks.sort_by(|a, b| compare(&a.0, &b.0));

    let matches = new_blocks
        .iter()
        .filter(|item| item.0 == new_blocks[0].0)
        .collect::<Vec<_>>();

    if matches.len() == 1 {
        return matches[0].1 .0.clone();
    }
    let next_blocks = matches
        .iter()
        .filter(|block| block.1 .1.len() > 1)
        .map(|block| (block.1 .0.to_string(), block.1 .1[1..].to_vec()))
        .collect::<Vec<_>>();
    if next_blocks.len() > 1 {
        return get_closest_match(lab, next_blocks);
    }
    matches[0].1 .0.clone()
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
