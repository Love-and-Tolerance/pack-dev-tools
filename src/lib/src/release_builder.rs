use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ValueEnum)]
pub enum MinecraftPlatform {
	Java,
	Bedrock,
	Both,
}

pub async fn release_builder() -> Result<(), Box<dyn std::error::Error>> {
	let bedrock = reqwest::get(
		"https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/bedrock.json",
	)
	.await?
   .json::<BedrockAssets>()
   .await?;
	println!("{:#?}", bedrock);
	Ok(())
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
pub struct Templates {
	asset_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockRepo {
	base: BedrockBaseRepo,
	addons: Vec<BedrockAddon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BedrockAssets {
	templates: Templates,
	repos: BedrockRepo,
}
