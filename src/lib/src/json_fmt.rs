use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::{error::Error, fs};

pub fn format_json_old() {
   let json_file = "./chiseled_bookshelf.json";
   let json_data = fs::read_to_string(json_file).expect("Failed to read file to string.");
   let json = serde_json::from_str::<Value>(&json_data).unwrap();

   std::fs::write(
      "./chiseled_bookshelf_fmt.json",
      serde_json::to_string_pretty(&json).unwrap(),
   )
   .unwrap();

   std::fs::write(
      "./chiseled_bookshelf_mini.json",
      serde_json::to_string(&json).unwrap(),
   )
   .unwrap();
}

#[inline]
pub fn parse_to_value(json: &str) -> Result<Value, Box<dyn Error>> {
   Ok(serde_json::from_str(json)?)
}

pub fn format_json(json: &str) -> Result<String, Box<dyn Error>> {
   let value = parse_to_value(json)?;
   let mut writer = Vec::with_capacity(256);
   let formatter = PrettyFormatter::with_indent(b"\t");
   let mut serialiser = Serializer::with_formatter(&mut writer, formatter);
   value.serialize(&mut serialiser)?;
   Ok(String::from_utf8(writer)?)
}

pub fn minify_json(json: &str) -> Result<String, Box<dyn Error>> {
   let value = parse_to_value(json)?;
   Ok(serde_json::to_string(&value)?)
}
