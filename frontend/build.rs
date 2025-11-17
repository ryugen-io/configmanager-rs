mod build_helpers;

use build_helpers::{date, hash, theme, version};

fn main() {
    // Extract dependency versions from Cargo.toml
    version::extract_dependency_versions();

    // Set build metadata
    date::set_build_date();
    hash::set_build_hash();

    // Load theme configuration
    theme::load_theme_config();

    // Rerun if files change
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=../sys/theme/theme.toml");
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/refs/heads");
}
