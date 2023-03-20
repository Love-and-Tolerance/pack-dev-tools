use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ValueEnum)]
pub enum MinecraftPlatform {
	Java,
	Bedrock,
	Both,
}

pub async fn release_builder(
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

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaBaseRepo {
	mc_versions: String,
	pack_format: String,
	version: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
	trigger: String,
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LicenseValue {
	Boolean(bool),
	String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseCondition {
	trigger: String,
	value: LicenseValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddonUrl {
	name: String,
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaAddonLink {
	name: String,
	url: JavaAddonUrl,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaVariant {
	name: String,
	id: String,
	image: Option<String>,
	description: Option<String>,
	url: Option<String>,
	branch: Option<JavaConditionalBranch>,
	trigger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaVariantAddon {
	name: String,
	id_pos: u32,
	apply_order: u32,
	default_variant: String,
	variants: Vec<JavaVariant>,
	license: Option<JavaConditionalLicense>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaBasicAddon {
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
pub struct JavaTemplates {
	zips_path: String,
	base_zip_name: String,
	variant_addon_zip_name: String,
	regular_addon_zip_name: String,
	mod_addon_zip_name: String,
	filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaAddons {
	exclusive: Vec<JavaVariantAddon>,
	regular: Vec<JavaBasicAddon>,
	mods: Vec<JavaBasicAddon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaRepo {
	base: JavaBaseRepo,
	addons: JavaAddons,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaAssets {
	templates: JavaTemplates,
	repos: JavaRepo,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JavaConditionalBranch {
	String(String),
	Conditions(Vec<Condition>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JavaConditionalLicense {
	Boolean(bool),
	Conditions(Vec<LicenseCondition>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JavaAddonUrl {
	String(String),
	URLs(Vec<AddonUrl>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockBaseRepo {
	mc_versions: String,
	pack_format: String,
	tag: String,
	version: String,
	filename: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockAddon {
	name: String,
	filename: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockTemplates {
	asset_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockRepo {
	base: BedrockBaseRepo,
	addons: Vec<BedrockAddon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockAssets {
	templates: BedrockTemplates,
	repos: BedrockRepo,
}
