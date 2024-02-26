use filter_org_txt::load_patterns_from_toml;

fn main() {
    let file_path = "/Users/toeinriver/Documents/pandoc-server-py/patterns.toml";
    match load_patterns_from_toml(file_path) {
        Ok(patterns) => {
            for pattern in patterns {
                println!("pattern: {:?}", pattern);
            }
        }
        Err(e) => eprintln!("Error loading patterns: {}", e),
    }
}
