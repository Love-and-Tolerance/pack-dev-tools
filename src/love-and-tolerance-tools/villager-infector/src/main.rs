use clap::Parser;
use pdtlib::villager_infector::infect_villagers;
use pdtlib::{pdtstdin, pdttrait::Vector};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Infect villager ponies.

example: .{SLASH}villager-infector .{SLASH}assets{SLASH}minelittlepony{SLASH}textures{SLASH}entity/pony
example: .{SLASH}villager-infector -rp .{SLASH}Community-Skin-Pack"),
	long_about = None)
]

struct Args {
	#[arg(short, long)]
	/// Convert Community Skin Pack into Villager Skin Pack
	resource_pack: bool,
	/// List of files and folders to infect.
	input_paths: Vec<String>,
}

fn main() {
	let overlay = include_bytes!("zompony_overlay.png");
	let args = Args::parse();
	let paths = pdtstdin::get_stdin()
		.unwrap_or(Vec::new())
		.extend_vec(args.input_paths);
	infect_villagers(paths, overlay, args.resource_pack);
}
