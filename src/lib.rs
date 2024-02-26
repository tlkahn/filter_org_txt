use regex::Regex;
use std::collections::HashMap;

pub fn load_patterns_from_toml(file: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file)?;
    let value: toml::Value = toml::from_str(&contents)?;
    let patterns = value["patterns"].as_array().ok_or("patterns is not an array")?
        .iter()
        .map(|v| {
            let mut pattern_map = HashMap::new();
            pattern_map.insert("pattern".to_string(), v["pattern"].as_str().unwrap().to_string());
            pattern_map.insert("replacement".to_string(), v["replacement"].as_str().unwrap().to_string());
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
