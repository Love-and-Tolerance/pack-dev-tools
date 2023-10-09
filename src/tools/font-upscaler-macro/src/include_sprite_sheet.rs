use ahash::{ HashMapExt as _, RandomState };
use image::GenericImageView as _;
use image::Rgba;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fmt;

type Map<K, V> = HashMap<K, V, RandomState>;

const IMAGE: &[u8] = include_bytes!("./transform-map.png");

pub fn process(_input: TokenStream) -> TokenStream {
	let transform_map = image::load_from_memory_with_format(IMAGE, image::ImageFormat::Png)
		.unwrap();

	{
		let mut footgun_pixels = Vec::new();
		for (x, y, pixel) in transform_map.pixels() {
			let [_, _, _, a] = pixel.0;

			if a != 0 && a != u8::MAX {
				footgun_pixels.push((x, y));
			}
		}

		if !footgun_pixels.is_empty() {
			let pixels = footgun_pixels.iter().fold(
				Vec::with_capacity(footgun_pixels.len()),
				|mut pixels, (x, y)| {
					pixels.push(format!("({x}, {y})"));
					pixels
				}
			)
			.join(", ");

			let num = footgun_pixels.len();
			let plural = if num == 1 { "pixel" } else { "pixels" };
			let message = format!("{num} problematic {plural} found! make sure these pixels are either fully opaque or fully transparent.\ncoords: {pixels}");

			return quote! {
				compile_error!(#message);
			}
		}
	}

	let cell_height = 4usize;
	let cell_width = 8usize;

	let height = transform_map.height() as usize;
	let width = transform_map.width() as usize;
	let num_pixels = height * width;

	assert_eq!(height % cell_height, 0, "transformation_map height multiple of {cell_height}");
	assert_eq!(width % cell_width, 0, "transformation_map width multiple of {cell_width}");

	let num_cells_height = height / cell_height;
	let num_cells_width = width / cell_width;

	let num_cells = num_cells_height * num_cells_width;

	let pixels = transform_map.pixels().collect::<Vec<_>>();
	assert_eq!(pixels.len(), num_pixels, "num_pixels");

	let cells_vec = pixels.into_iter()
		.chunks(width) // divide it into rows
		.into_iter()
		.chunks(cell_height) // group rows by cell_height
		.into_iter()
		.flat_map(|row_group| {
			let mut row_group = row_group.collect::<Vec<_>>();

			let mut cells = Vec::with_capacity(num_cells_width);
			let mut next_cell = Vec::with_capacity(cell_height * cell_width);

			for _ in 0..num_cells_width {
				for row in row_group.iter_mut() {
					for _ in 0..cell_width {
						next_cell.push(row.next().unwrap());
					}
				}

				cells.push(next_cell);
				next_cell = Vec::with_capacity(cell_height * cell_width);
			}

			cells
		})
		.collect::<Vec<_>>();

	assert_eq!(cells_vec.len(), num_cells);

	let mut cells_map = Map::<String, (Pixel, UpscaledPixel)>::new();
	let mut transformed_map = Map::<String, (Pixel, UpscaledPixel)>::new();
	let mut dupes = Map::<String, (Pixel, usize)>::new();

	let mut insert_into_map = |map: &mut Map<String, (Pixel, UpscaledPixel)>, k: String, v: (Pixel, UpscaledPixel), insert_to_dupes: bool| {
		if map.contains_key(&k) {
			if insert_to_dupes {
				if let Some(dupe) = dupes.get_mut(&k) {
					dupe.1 += 1;
				} else {
					dupes.insert(k, (v.0, 1));
				}
			}
			return
		}

		map.insert(k, v);
	};

	let cell_iter = cells_vec.into_iter()
		.filter(|c| c.iter().any(|p| s(*p) == PTrue));
	for cell in cell_iter {
		assert_eq!(cell.len(), cell_height * cell_width);
		// 00 01 02 03 04 05 06 07
		// 08 09 10 11 12 13 14 15
		// 16 17 18 19 20 21 22 23
		// 24 25 26 27 28 29 30 31


		// That there is the pixel
		//     |
		//     |
		//     V
		// 00 01 02 | 03 04 05 06 07
		// 08 09 10 | 11 12 13 14 15
		// 16 17 18 | 19 20 21 22 23
		// ---------
		// 24 25 26   27 28 29 30 31


		// that there is the upscaled pixel
		//                    |
		//                    |
		//                    V
		// 00 01 02 03 | 04 05 06 07
		// 08 09 10 11 | 12 13 14 15
		// 16 17 18 19 | 20 21 22 23
		// 24 25 26 27 | 28 29 30 31


		let pixel: Pixel = [
			[s(cell[0]), s(cell[1]), s(cell[2])],
			[s(cell[8]), s(cell[9]), s(cell[10])],
			[s(cell[16]), s(cell[17]), s(cell[18])],
		];
		let upscaled: UpscaledPixel = [
			[s(cell[4]), s(cell[5]), s(cell[6]), s(cell[7])],
			[s(cell[12]), s(cell[13]), s(cell[14]), s(cell[15])],
			[s(cell[20]), s(cell[21]), s(cell[22]), s(cell[23])],
			[s(cell[28]), s(cell[29]), s(cell[30]), s(cell[31])]
		];
		let none = (pixel, upscaled);

		let mut flipped = none;
		flip(&mut flipped.0);
		flip(&mut flipped.1);

		let mut rotated90 = none;
		rotate(&mut rotated90.0);
		rotate(&mut rotated90.1);
		let mut rotated90_flipped = flipped;
		rotate(&mut rotated90_flipped.0);
		rotate(&mut rotated90_flipped.1);


		let mut rotated180 = rotated90;
		rotate(&mut rotated180.0);
		rotate(&mut rotated180.1);
		let mut rotated180_flipped = rotated90_flipped;
		rotate(&mut rotated180_flipped.0);
		rotate(&mut rotated180_flipped.1);

		let mut rotated270 = rotated180;
		rotate(&mut rotated270.0);
		rotate(&mut rotated270.1);
		let mut rotated270_flipped = rotated180_flipped;
		rotate(&mut rotated270_flipped.0);
		rotate(&mut rotated270_flipped.1);

		let transformed_cells = [
			flipped,
			rotated90,
			rotated90_flipped,
			rotated180,
			rotated180_flipped,
			rotated270,
			rotated270_flipped,
		];

		let none_key = none.0.to_key();
		insert_into_map(&mut cells_map, none_key, none, true);

		for cell in transformed_cells {
			let k = cell.0.to_key();
			insert_into_map(&mut transformed_map, k, cell, false);
		}
	}

	for (k, v) in transformed_map.into_iter() {
		insert_into_map(&mut cells_map, k, v, false);
	}

	let mut missing = Vec::with_capacity(num_cells);
	let states = [PTrue, PFalse];

	// nested loop of doom
	for t in states.into_iter() {
		for b in states.into_iter() {
			for l in states.into_iter() {
				for r in states.into_iter() {
					for tl in states.into_iter() {
						for tr in states.into_iter() {
							for bl in states.into_iter() {
								for br in states.into_iter() {
									for m in states.into_iter() {
										let pixel: Pixel = [
											[tl, t, tr],
											[l, m, r],
											[bl, b, br]
										];
										let key = pixel.to_key();
										if !cells_map.contains_key(&key) {
											missing.push(pixel);
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}

	let mut panic_msg = None;

	if !dupes.is_empty() {
		panic_msg = Some(dupes.iter().map(|(_, (dupe, _))| dupe.get_nice_grid("Duplicate entry!")).join(""));
	}

	if !missing.is_empty() {
		let msg = missing.iter().map(|missing| missing.get_nice_grid("Missing entry!")).join("");
		panic_msg = if let Some(panic_msg) = panic_msg {
			Some(format!("{panic_msg}=============================================\n{msg}"))
		} else {
			Some(msg)
		};
	}

	if let Some(panic_msg) = panic_msg {
		let cells_len = cells_map.len();
		let dupes_len = dupes.values().fold(0usize, |curr, (_, c)| curr + c);
		let missing_len = missing.len();
		let panic_msg = format!("{panic_msg}   {cells_len} cells, {dupes_len} dupes, {missing_len} missing");
		return quote! {
			compile_error!(#panic_msg);
		}
	}

	_input
}

type Pixel = [[PixelState; 3]; 3];
type UpscaledPixel = [[PixelState; 4]; 4];

use PixelState::*;
#[derive(Clone, Copy, PartialEq, Eq)]
enum PixelState {
	PTrue,
	PFalse
}

// this is kinda dirty and not how you're supposed to use Display/Debug
// but exploit the language, lol

impl fmt::Display for PixelState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			PTrue => { '1' }
			PFalse => { '0' }
		})
	}
}

impl fmt::Debug for PixelState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			PTrue => { '0' }
			PFalse => { '_' }
		})
	}
}

trait ToKey {
	fn to_key(&self) -> String;
	fn get_nice_grid(&self, msg: &str) -> String;
}

impl ToKey for Pixel {
	fn to_key(&self) -> String {
		let [
			[tl, t, tr],
			[l, m, r],
			[bl, b, br]
		] = self;
		format!("{t}{b}{l}{r}{tl}{tr}{bl}{br}{m}")
	}

	fn get_nice_grid(&self, msg: &str) -> String {
		let [
			[tl, t, tr],
			[l, m, r],
			[bl, b, br]
		] = self;

		let line1 = format!("   {msg}");
		let line2 = format!("   {tl:?}{t:?}{tr:?}");
		let line3 = format!("   {l:?}{m:?}{r:?}");
		let line4 = format!("   {bl:?}{b:?}{br:?}");
		format!("{line1}\n{line2}\n{line3}\n{line4}\n\n")
	}
}

fn s(pixel: (u32, u32, Rgba<u8>)) -> PixelState {
	let alpha = pixel.2.0[3];
	match alpha {
		0 => { PFalse }
		u8::MAX => { PTrue }
		_ => { unreachable!("pixel should have been checked for valid opacity AAAaaaaaaAAaAAAaaAAaaAAaAAAAAAAAaaaaaAAaaAAaaaaa") }
	}
}

fn flip<T: Clone, const N: usize>(vec: &mut [[T; N]; N]) {
	for i in 0..vec.len() {
		for h in i..vec.len() {
			let loc1 = vec.get_mut(i).unwrap().get_mut(h).unwrap();
			let mut temp = loc1.clone();
			std::mem::swap(&mut temp, loc1);

			let loc2 = vec.get_mut(h).unwrap().get_mut(i).unwrap();
			std::mem::swap(&mut temp, loc2);

			let loc1 = vec.get_mut(i).unwrap().get_mut(h).unwrap();
			std::mem::swap(&mut temp, loc1);
		}
	}
}

fn rotate<T: Clone, const N: usize>(vec: &mut [[T; N]; N]) {
	vec.reverse();
	flip(vec);
}
