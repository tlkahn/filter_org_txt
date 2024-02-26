use filter_org_txt::{apply_patterns, load_patterns_from_toml};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let patterns_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("patterns.toml");
    let patterns = load_patterns_from_toml(patterns_path.to_str().unwrap())?;

    let input_file_path = env::args().nth(1).expect("Missing input file path");
    let text = std::fs::read_to_string(&input_file_path)?;

    let updated_text = apply_patterns(&patterns, &text);

    let output_file_path = env::args()
        .nth(2)
        .unwrap_or_else(|| input_file_path.clone());
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(updated_text.as_bytes())?;

    Ok(())
}
