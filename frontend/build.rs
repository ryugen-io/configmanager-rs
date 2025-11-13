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

    // Rerun if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
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
