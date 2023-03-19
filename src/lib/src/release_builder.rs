use clap::ValueEnum;
use std::collections::HashMap;

#[derive(Clone, Debug, ValueEnum)]
pub enum MinecraftPlatform {
	Java,
	Bedrock,
	Both,
}

pub async fn release_builder() -> Result<(), Box<dyn std::error::Error>> {
	let resp = reqwest::get(
		"https://github.com/Love-and-Tolerance/pack-builder-assets/raw/mane/assets/java.json",
	)
	.await?
	.json::<HashMap<String, String>>()
	.await?;
	println!("{:#?}", resp);
	Ok(())
}
