use filter_org_txt::{apply_patterns, load_patterns_from_toml};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

fn get_example_toml_path(filename: &str) -> PathBuf {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push("fixtures");
    path.push(filename);
    path
}

#[test]
fn test_load_patterns_from_toml() {
    let file_path = get_example_toml_path("example.toml");
    let result = load_patterns_from_toml(file_path.to_str().expect("Invalid file path"))
        .expect("Failed to load patterns from TOML file");

    assert_eq!(
        result,
        vec!["rust".to_string(), "toml".to_string(), "test".to_string()]
    );
}
#[test]
fn test_load_patterns_from_nonexistent_file() {
    let result = load_patterns_from_toml("nonexistent.toml");
    assert!(result.is_err());
}
#[test]
fn test_load_patterns_from_invalid_toml() {
    let file_path = get_example_toml_path("invalid.toml");
    std::fs::write(&file_path, "invalid = :toml").unwrap();

    let result = load_patterns_from_toml(&file_path.to_str().expect("Invalid file path"));
    assert!(result.is_err());
}
#[test]
fn test_apply_patterns() {
    let mut pattern1 = HashMap::new();
    pattern1.insert("pattern".to_string(), r"\d+".to_string());
    pattern1.insert("replacement".to_string(), "NUM".to_string());

    let mut pattern2 = HashMap::new();
    pattern2.insert("pattern".to_string(), r"\s+".to_string());
    pattern2.insert("replacement".to_string(), "_".to_string());

    let patterns = vec![pattern1, pattern2];
    let text = "I have 123 apples and 456 oranges.";

    let result = apply_patterns(&patterns, &text);
    assert_eq!(result, "I_have_NUM_apples_and_NUM_oranges.");
}
