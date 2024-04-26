use base64;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

fn clean_svg_xml_suffix(s: &str) -> &str {
    if s == "svg+xml" {
        "svg"
    } else {
        s
    }
}

pub fn replace_svg_xml_with_links(img_dir: &str, s: &str) -> (String, String) {
    let pattern = r"\[\[data:(//)?image/(svg\+xml|png);base64\,([^\]]+?)\]\]";
    let b64_pattern = Regex::new(pattern).unwrap();

    let mut modified_s = s.to_string();
    let mut g_img_path = String::new();

    for captures in b64_pattern.captures_iter(s) {
        let slashes = captures.get(1).map_or("", |m| m.as_str());
        let suffix = captures.get(2).map_or("", |m| m.as_str());
        let b64_string = captures.get(3).map_or("", |m| m.as_str());
        let id = Uuid::new_v4();
        let filename = format!("{}/{}", img_dir, id.to_string());
        let img_path =
            PathBuf::from(img_dir).join(format!("{}.{}", filename, clean_svg_xml_suffix(suffix)));

        let img_data = base64::decode(b64_string).unwrap();
        let mut f = File::create(&img_path).unwrap();
        f.write_all(&img_data).unwrap();

        let img_link = format!("[[{}]]", img_path.display());
        modified_s = modified_s.replace(
            &format!("[[data:{}image/{};base64,{}]]", slashes, suffix, b64_string),
            &img_link,
        );
        g_img_path = img_path.to_str().unwrap().to_string();
    }

    (modified_s, g_img_path)
}

pub fn load_patterns_from_toml(
    file: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file)?;
    let value: toml::Value = toml::from_str(&contents)?;
    let patterns = value["patterns"]
        .as_array()
        .ok_or("patterns is not an array")?
        .iter()
        .map(|v| {
            let mut pattern_map = HashMap::new();
            pattern_map.insert(
                "pattern".to_string(),
                v["pattern"].as_str().unwrap().to_string(),
            );
            pattern_map.insert(
                "replacement".to_string(),
                v["replacement"].as_str().unwrap().to_string(),
            );
            pattern_map
        })
        .collect();

    Ok(patterns)
}
pub fn apply_patterns(patterns: &[HashMap<String, String>], text: &str) -> String {
    patterns.iter().fold(text.to_string(), |acc, p| {
        if let (Some(pattern), Some(replacement)) = (p.get("pattern"), p.get("replacement")) {
            let re = Regex::new(pattern).unwrap();
            re.replace_all(&acc, replacement.as_str()).to_string()
        } else {
            acc
        }
    })
}
