// use std::fs;
// use std::fs::File;
// use std::io::Write;
// use flate2::write::GzEncoder;
// use flate2::Compression;
// use serde_json::Value;
//
//
// /// Compress JSON file
// /// Simply read the json and write it to a gzipped file
// #[allow(dead_code)]
// fn compress_json_file(input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // Read the JSON file
//     let data = fs::read_to_string(input_path)?;
//     let json_value: Value = serde_json::from_str(&data)?;
//
//     // Write json to string
//     let content = serde_json::to_string(&json_value)?;
//
//     // Write the JSON file to a gzipped file
//     let output_path = format!("{}.gz", input_path);
//     let file = File::create(output_path)?;
//     let mut encoder = GzEncoder::new(file, Compression::default());
//     encoder.write_all(content.as_bytes())?;
//
//     Ok(())
// }
//
fn main() {

}
