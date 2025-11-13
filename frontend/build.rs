use std::fs;

fn main() {
    // Parse Cargo.toml to extract dependency versions
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    // Extract ratzilla version
    if let Some(ratzilla_line) = cargo_toml.lines().find(|line| line.contains("ratzilla"))
        && let Some(version) = extract_version(ratzilla_line)
    {
        println!("cargo:rustc-env=RATZILLA_VERSION={}", version);
    }

    // Extract tui-textarea version (which depends on ratatui)
    if let Some(textarea_line) = cargo_toml
        .lines()
        .find(|line| line.contains("tui-textarea"))
        && let Some(_version) = extract_version(textarea_line)
    {
        // tui-textarea 0.7 uses ratatui 0.29
        println!("cargo:rustc-env=RATATUI_VERSION=0.29");
    }

    // Axum version from server
    println!("cargo:rustc-env=AXUM_VERSION=0.7");

    // Load theme configuration
    load_theme_config();

    // Rerun if files change
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=theme.toml");
}

fn extract_version(line: &str) -> Option<String> {
    // Try to extract version from lines like: ratzilla = "0.2"
    if let Some(start) = line.find('"')
        && let Some(end) = line[start + 1..].find('"')
    {
        return Some(line[start + 1..start + 1 + end].to_string());
    }
    // Try to extract from version = "x.y" format
    if let Some(start) = line.find("version") {
        let rest = &line[start..];
        if let Some(quote_start) = rest.find('"')
            && let Some(quote_end) = rest[quote_start + 1..].find('"')
        {
            return Some(rest[quote_start + 1..quote_start + 1 + quote_end].to_string());
        }
    }
    None
}

fn load_theme_config() {
    let theme_content = fs::read_to_string("theme.toml").expect("Failed to read theme.toml");
    let theme: toml::Value = toml::from_str(&theme_content).expect("Failed to parse theme.toml");

    // Extract color values
    let colors = theme
        .get("colors")
        .and_then(|c| c.as_table())
        .expect("Missing [colors] section in theme.toml");

    // Set environment variables for each color
    for (name, value) in colors {
        if let Some(rgb_array) = value.as_array()
            && rgb_array.len() == 3
        {
            let r = rgb_array[0].as_integer().expect("Invalid RGB value") as u8;
            let g = rgb_array[1].as_integer().expect("Invalid RGB value") as u8;
            let b = rgb_array[2].as_integer().expect("Invalid RGB value") as u8;
            let env_name = format!("THEME_COLOR_{}", name.to_uppercase());
            println!("cargo:rustc-env={}={},{},{}", env_name, r, g, b);
        }
    }
}
