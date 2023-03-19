use clap::ValueEnum;
use serde_json;

#[derive(Clone, Debug, ValueEnum)]
pub enum MinecraftPlatform {
	Java,
	Bedrock,
	Both,
}

pub async fn release_builder() -> Result<(), Box<dyn std::error::Error>> {
	let resp = reqwest::get(
		"https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/java.json",
	)
	.await?
   .text()
   .await?;
   let json = serde_json::from_str(&resp)?;
	println!("{:?}", json);
	Ok(())
}
