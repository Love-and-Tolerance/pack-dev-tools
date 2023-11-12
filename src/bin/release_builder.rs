use clap::Parser;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::MAIN_SEPARATOR as SLASH;

#[derive(Clone, Debug, ValueEnum)]
pub enum MinecraftPlatform {
	Java,
	Bedrock,
	Both,
}

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

async fn release_builder(
	platform: Option<MinecraftPlatform>,
) -> Result<(), Box<dyn std::error::Error>> {
	if platform.is_some() {
		println!("{:#?}", platform);
	}
	let bedrock = reqwest::get(
		"https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/bedrock.json",
	)
	.await?
   .json::<BedrockAssets>()
   .await?;
	println!("{:#?}", bedrock);
	let java = reqwest::get(
		"https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/java.json",
	)
	.await?
   .json::<JavaAssets>()
   .await?;
	println!("{:#?}", java);
	Ok(())
}

// java structs
#[derive(Serialize, Deserialize, Debug)]
struct JavaAssets {
	templates: JavaTemplates,
	repos: JavaRepo,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaTemplates {
	zips_path: String,
	base_zip_name: String,
	variant_addon_zip_name: String,
	regular_addon_zip_name: String,
	mod_addon_zip_name: String,
	filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaRepo {
	base: JavaBaseRepo,
	addons: JavaAddons,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaBaseRepo {
	mc_versions: String,
	pack_format: String,
	version: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaAddons {
	exclusive: Vec<JavaVariantAddon>,
	regular: Vec<JavaBasicAddon>,
	mods: Vec<JavaBasicAddon>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaVariantAddon {
	name: String,
	id_pos: u32,
	apply_order: u32,
	default_variant: String,
	variants: Vec<JavaVariant>,
	license: Option<JavaConditionalLicense>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaVariant {
	name: String,
	id: String,
	image: Option<String>,
	description: Option<String>,
	url: Option<String>,
	branch: Option<JavaConditionalBranch>,
	trigger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum JavaConditionalBranch {
	String(String),
	Conditions(Vec<Condition>),
}

#[derive(Serialize, Deserialize, Debug)]
struct Condition {
	trigger: String,
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum JavaConditionalLicense {
	Boolean(bool),
	Conditions(Vec<LicenseCondition>),
}

#[derive(Serialize, Deserialize, Debug)]
struct LicenseCondition {
	trigger: String,
	value: LicenseValue,
}

#[derive(Serialize, Deserialize, Debug)]
enum LicenseValue {
	Boolean(bool),
	String(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaBasicAddon {
	id: String,
	name: String,
	recommended: bool,
	url: String,
	description: Option<String>,
	info: Option<Vec<String>>,
	links: Option<Vec<JavaAddonLink>>,
	branch: Option<JavaConditionalBranch>,
	license: Option<JavaConditionalLicense>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JavaAddonLink {
	name: String,
	url: JavaAddonUrl,
}

#[derive(Serialize, Deserialize, Debug)]
enum JavaAddonUrl {
	String(String),
	URLs(Vec<AddonUrl>),
}

#[derive(Serialize, Deserialize, Debug)]
struct AddonUrl {
	name: String,
	value: String,
}

// Bedrock structs
#[derive(Serialize, Deserialize, Debug)]
struct BedrockAssets {
	templates: BedrockTemplates,
	repos: BedrockRepo,
}

#[derive(Serialize, Deserialize, Debug)]
struct BedrockTemplates {
	asset_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BedrockRepo {
	base: BedrockBaseRepo,
	addons: Vec<BedrockAddon>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BedrockBaseRepo {
	mc_versions: String,
	pack_format: String,
	tag: String,
	version: String,
	filename: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BedrockAddon {
	name: String,
	filename: String,
	url: String,
}
