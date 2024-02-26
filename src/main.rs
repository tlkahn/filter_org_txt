use filter_org_txt::{apply_patterns, load_patterns_from_toml, replace_svg_xml_with_links};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let patterns_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("patterns.toml");
    let patterns = load_patterns_from_toml(patterns_path.to_str().unwrap())?;

    let input_file_path = env::args().nth(1).expect("Missing input file path");
    let text = std::fs::read_to_string(&input_file_path)?;
    let img_dir = format!("{}/Documents/org/web/WebImg", std::env::var("HOME").unwrap());

    let updated_text = apply_patterns(&patterns, &text);
    let (updated_text, _) = replace_svg_xml_with_links(&img_dir, &updated_text);

    let output_file_path = env::args()
        .nth(2)
        .unwrap_or_else(|| input_file_path.clone());
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(updated_text.as_bytes())?;

    Ok(())
}
