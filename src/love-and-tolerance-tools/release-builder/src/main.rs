use clap::Parser;
use pdtlib::release_builder::{release_builder, MinecraftPlatform};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"),
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = format!("Build Love & Tolerance release.

example: .{SLASH}release-builder"),
	long_about = None)
]

struct Args {
	#[arg(short, long)]
	/// Minecraft platform
	platform: Option<MinecraftPlatform>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	release_builder(args.platform).await?;
	Ok(())
}
