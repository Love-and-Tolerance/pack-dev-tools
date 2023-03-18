use clap::Parser;
use pdtlib::villager_infector::infect_villagers;
use pdtlib::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Infect villager ponies.

example: .{s}villager-infector .{s}assets{s}minelittlepony{s}textures{s}entity/pony
example: .{s}villager-infector -c .{s}Community-Skin-Pack", s = SLASH),
	long_about = None)
]

struct Args {
	#[arg(short, long)]
	/// Convert Community Skin Pack into Villager Skin Pack.
	convert: bool,
	/// List of files and folders to infect.
	input_paths: Vec<String>,
}

fn main() {
	let overlay = include_bytes!("zompony_overlay.png");
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or(Vec::new())
		.extend_vec(args.input_paths);
	infect_villagers(paths, overlay, args.convert);
}
