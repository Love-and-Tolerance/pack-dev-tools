use ahash::{ HashMapExt as _, HashSetExt as _, RandomState };
use image::GenericImageView as _;
use image::Rgba;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{ HashMap, HashSet };
use std::fmt;

type Map<K, V> = HashMap<K, V, RandomState>;
type Set<T> = HashSet<T, RandomState>;

const IMAGE: &[u8] = include_bytes!("./transform-map.png");

pub fn process(_input: TokenStream) -> TokenStream {
	let transform_map = image::load_from_memory_with_format(IMAGE, image::ImageFormat::Png)
		.unwrap();

	// TODO: this needs to be updated/removed if/when we decided to do higher res upscaling
	let cell_height = 4usize;
	let cell_width = 8usize;

	let num_cells_height = 8usize;
	let num_cells_width = 8usize;

	let height = cell_height * num_cells_height;
	let width = cell_width * num_cells_width;

	assert_eq!(transform_map.height() as usize, height, "transformation_map height");
	assert_eq!(transform_map.width() as usize, width, "transformation_map width");
	let num_pixels = height * width;

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

	let mut cells_map = Map::new();
	let mut dupes = Vec::with_capacity(num_cells);

	for cell in cells_vec.into_iter() {
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



		let pixel = Pixel {
			t: s(cell[1]),
			b: s(cell[17]),
			l: s(cell[8]),
			r: s(cell[10]),
			tl: s(cell[0]),
			tr: s(cell[2]),
			bl: s(cell[16]),
			br: s(cell[18]),
			m: s(cell[9])
		};
		let pixel_key = pixel.to_key();
		if cells_map.contains_key(&pixel_key) {
			if !dupes.contains(&pixel) {
				dupes.push(pixel);
			}
			continue
		}
		cells_map.insert(pixel_key, pixel);
	}

	// nested loop of doom
	let mut missing = Vec::with_capacity(num_cells - cells_map.len());

	let states = [PTrue, PFalse];

	for t in states.into_iter() {
		for b in states.into_iter() {
			for l in states.into_iter() {
				for r in states.into_iter() {
					for tl in states.into_iter() {
						for tr in states.into_iter() {
							for bl in states.into_iter() {
								for br in states.into_iter() {
									for m in states.into_iter() {
										let pixel = Pixel { t, b, l, r, tl, tr, bl, br, m };
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
		panic_msg = Some(dupes.iter().map(|dupe| dupe.get_nice_grid("Duplicate entry!")).join(""));
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
		return quote! {
			compile_error!(#panic_msg);
		}
	}


	// // vec of rows
	// let thing = pixels.into_iter()
	// 	.chunks(width)
	// 	.into_iter()
	// 	.map(|c| c.collect::<Vec<_>>())
	// 	// .collect::<Vec<_>>();

	// // assert_eq!(rows.len(), height, "num rows");
	// // assert!(rows.iter().all(|r| r.len() == width), "num of all cols");

	// // vecs of vecs, each vec containing a group of rows by `cell_height`
	// // let grouped_rows = rows.into_iter()
	// 	.chunks(cell_height)
	// 	.into_iter()
	// 	.map(|c| c.collect::<Vec<_>>())

	// // each row now contains vecs of cells divided by width,
	// 	.map(|row_group| {
	// 		row_group.into_iter()
	// 			.map(|row| {
	// 				row.into_iter()
	// 					.chunks(cell_width)
	// 					.into_iter()
	// 					.map(|c| c.collect::<Vec<_>>())
	// 					.collect::<Vec<_>>()
	// 			})
	// 			.collect::<Vec<_>>()
	// 	})
	// 	.collect::<Vec<_>>();
	// 	// .collect::<Vec<_>>();
	// 	// NOW
	// 	// each row contains

	// // now we manipulate

	// // vecs of rows
	// let cells = thing.into_iter()
	// 	.map(|row_group| {
	// 		row_group.into_iter()

	// 			// .map(|row| {})
	// 	});

	// assert!(grouped_rows.iter().all(|r| r.len() == num_cells_height));


	// let grouped = rows.iter_mut().for_each(|me| {})

	_input
}

#[derive(PartialEq, Eq)]
struct Pixel {
	t: PixelState,
	b: PixelState,
	l: PixelState,
	r: PixelState,
	tl: PixelState,
	tr: PixelState,
	bl: PixelState,
	br: PixelState,
	m: PixelState
}

struct UpscaledPixel {
	r1: [PixelState; 4],
	r2: [PixelState; 4],
	r3: [PixelState; 4],
	r4: [PixelState; 4]

}

use PixelState::*;
#[derive(Clone, Copy, PartialEq, Eq)]
enum PixelState {
	PTrue,
	PFalse
}

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

impl Pixel {
	fn to_key(&self) -> String {
		let Pixel { t, b, l, r, tl, tr, bl, br, m } = self;
		format!("{t}{b}{l}{r}{tl}{tr}{bl}{br}{m}")
	}

	fn get_nice_grid(&self, msg: &str) -> String {
		let Pixel { t, b, l, r, tl, tr, bl, br, m } = self;

		let line1 = format!("   {msg}");
		let line2 = format!("   {tl:?}{t:?}{tr:?}");
		let line3 = format!("   {l:?}{m:?}{r:?}");
		let line4 = format!("   {bl:?}{b:?}{br:?}");
		format!("{line1}\n{line2}\n{line3}\n{line4}\n\n")
	}
}

fn s(pixel: (u32, u32, Rgba<u8>)) -> PixelState {
	if pixel.2.0[3] == 0 { PFalse } else { PTrue }
}
