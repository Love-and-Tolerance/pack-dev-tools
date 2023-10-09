use camino::Utf8Path;
use image::{ Rgba, GenericImageView as _};
use itertools::Itertools as _;
use pdtlib::pdtfs::{ create_output_dir, find_files_in_dir };
use pdtlib::pdtthread::multithread;
use std::fs;
use std::num::NonZeroUsize;

font_upscaler_macro::include_sprite_sheet!();

fn main() {
	let mut dirs = Vec::new();
	let mut exts = Vec::new();
	let mut output_dir = "out".to_string();
	let mut cell_height = NonZeroUsize::new(8).unwrap();
	let mut cell_width = NonZeroUsize::new(8).unwrap();

	let mut args = std::env::args();
	args.next().unwrap();
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"-i" | "--input-dir" => { dirs.push(args.next().expect("unexpected end of args (expected input dir)")) }
			"-e" | "--exts" => { exts.push(args.next().expect("unexpected end of args (expected file ext)")) }
			"-o" | "--output-dir" => { output_dir = args.next().expect("unexpected end of args (expected output dir)") }
			"-h" | "--height" => {
				cell_height = args.next()
					.expect("unexpected end of args (expected height)")
					.parse()
					.expect("expected positive integer for height")
			}
			"-w" | "--width" => {
				cell_width = args.next()
					.expect("unexpected end of args (expected width)")
					.parse()
					.expect("expected positive integer for width")
			}
			arg => { panic!("unrecognised arg: {arg}") }
		}
	}

	if dirs.is_empty() { dirs = vec!["in".into()] }
	if exts.is_empty() { exts = vec!["png".into()] }
	let cell_height = usize::from(cell_height);
	let cell_width = usize::from(cell_width);

	// let files = find_files_in_multiple_dirs(dirs, true, Some(exts), &false, false);
	let dirs = dirs.into_iter()
		.map(|d| (d, output_dir.clone(), exts.clone()))
		.collect();
	create_output_dir(&output_dir);

	multithread(dirs, None, move |tn, (in_dir, out_dir, exts)| {
		let files = find_files_in_dir(&in_dir, true, &Some(exts));
		let files = files.into_iter()
			.map(|f| (f, in_dir.clone(), out_dir.clone()))
			.collect::<Vec<_>>();

		let processed = multithread(files, None, move |tn2, (file, in_dir, out_dir)| {
			let img = image::open(&file).unwrap();
			let (height, width) = (img.height() as usize, img.width() as usize);

			let cells = img.pixels()
				.chunks(width)
				.into_iter()
				.chunks(cell_height)
				.into_iter()
				.flat_map(|row_group| {
					let mut row_group = row_group.collect::<Vec<_>>();

					let mut cells = Vec::with_capacity(width / cell_width);
					let mut next_cell = Vec::with_capacity(cell_height * cell_width);

					for _ in 0..(width / cell_width) {
						for row in row_group.iter_mut() {
							for _ in 0..cell_width {
								next_cell.push(row.next().unwrap());
							}
						}

						cells.push((cell_height, cell_width, next_cell));
						next_cell = Vec::with_capacity(cell_height * cell_width);
					}

					cells
				})
				.collect::<Vec<_>>();

			let upscaled_cells = multithread(cells, None, |tn3, (height, width, pixels)| {
				let cell = CellHelper { height, width, pixels };

				let upscaled = cell.get_pixels()
					.into_iter()
					.map(upscaler::match_pixel)
					.collect::<Vec<_>>();

				Some(upscaled.into_iter())
			})
				.into_iter() // iterates cells
				.chunks(width / cell_width)
				.into_iter() // iterates rows of cells
				.flat_map(|cell_row| {
					// ignore the fact that each pix in a cell is upscaled by 4,
					// treat it as a regular cell like at top
					let mut cell_row = cell_row.collect::<Vec<_>>();

					let mut rows_of_pixels = Vec::with_capacity(cell_height);
					let mut current_row = Vec::with_capacity(width);

					for _ in 0..cell_height {
						for cell in cell_row.iter_mut() {
							for _ in 0..cell_width {
								current_row.push(cell.next().unwrap())
							}
						}

						rows_of_pixels.push(current_row);
						current_row = Vec::with_capacity(width)
					}

					// back to groups of rows, like in the beginning
					// BUT
					// we can see it as rows of 4x4 groups
					rows_of_pixels.into_iter()
						.flat_map(|pixel_row| {
							let mut pixel_row = pixel_row.into_iter()
								.map(|p| p.into_iter())
								.collect::<Vec<_>>();
							let mut rows_of_upscaled_pixels = Vec::with_capacity(cell_height * 4);
							let mut current_row = Vec::with_capacity(width * 4);

							for _ in 0..4 {
								for pixel in pixel_row.iter_mut() {
									current_row.extend(pixel.next().unwrap().into_iter());
								}

								rows_of_upscaled_pixels.push(current_row);
								current_row = Vec::with_capacity(width * 4);
							}

							rows_of_upscaled_pixels
						})
						.flatten()
				}) // long boi row of smol pixels
				.flat_map(|p| if p {
					[u8::MAX, u8::MAX, u8::MAX, u8::MAX]
				} else {
					[u8::MAX, u8::MAX, u8::MAX, 0]
				})
				.collect::<Vec<_>>();

			let upscaled_img = image::ImageBuffer::<Rgba<u8>, _>::from_vec(width as u32 * 4, height as u32 * 4, upscaled_cells).unwrap();

			let file_no_dir = &file[in_dir.len()..];
			let mut file_dest = out_dir;
			file_dest.reserve(file_no_dir.len());
			file_dest.push_str(file_no_dir);
			let file_dest = Utf8Path::new(&file_dest);

			fs::create_dir_all(file_dest.parent().unwrap().as_str()).unwrap();
			upscaled_img.save(&file_dest).unwrap();

			Some((file, "e"))
		});


		None::<()>
	});

}

struct CellHelper {
	height: usize,
	width: usize,
	pixels: Vec<(u32, u32, Rgba<u8>)>
}

impl CellHelper {
	fn is_present_with_overflow(&self, x: isize, y: isize) -> bool {
		let res = if x < 0 || y < 0 || x >= self.width as _ || y >= self.height as _ {
			// println!("({x}, {y}): false OOB, h{} w{}", self.height, self.width);
			false
		} else {
			let pixel = self.pixels.get(x as usize + (y as usize * self.width)).unwrap();
			match pixel.2.0[3] {
				0 => { /* println!("({x}, {y}): false, h{} w{}", self.height, self.width); */ false }
				u8::MAX => { /* println!("({x}, {y}): true, h{} w{}", self.height, self.width); */ true }
				_ => { panic!("pixel not fully opaque/transparent") }
			}
		};
		res
	}
	fn get_pixels(&self) -> Vec<upscaler::Pixel> {
		self.pixels.iter()
			.enumerate()
			.map(|(i, _)| i as isize)
			.map(|i| {
				let (x, y) = (i % self.width as isize, i / self.width as isize);
				[
					[
						self.is_present_with_overflow(x - 1, y - 1),
						self.is_present_with_overflow(x, y - 1),
						self.is_present_with_overflow(x + 1, y - 1)
					],
					[
						self.is_present_with_overflow(x - 1, y),
						self.is_present_with_overflow(x, y),
						self.is_present_with_overflow(x + 1, y)
					],
					[
						self.is_present_with_overflow(x - 1, y + 1),
						self.is_present_with_overflow(x, y + 1),
						self.is_present_with_overflow(x + 1, y + 1)
					]
				]
			})
			.collect()
	}
}

// fn chunk(h: usize, w: usize) {}
