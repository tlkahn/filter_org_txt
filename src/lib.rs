use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toml::Value;

pub fn load_patterns_from_toml(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let value: Value = toml::from_str(&contents)?;
    let patterns = value
        .get("patterns")
        .ok_or("patterns key not found")?
        .as_array()
        .ok_or("patterns is not an array")?
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
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
