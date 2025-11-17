use std::path::PathBuf;

/// Load keybinds configuration with XDG compliance support.
///
/// Order of precedence:
/// 1. User-specified keybinds file (USER_KEYBINDS_FILE env var)
/// 2. Default frontend/keybinds.toml
///
/// The selected file path is set as KEYBINDS_FILE env var for
/// embedding into the WASM binary. Path must be relative to
/// frontend/src/keybinds/mod.rs (where include_str! is called).
pub fn load_keybinds_config() {
    const BLUE: &str = "\x1b[38;2;137;180;250m";
    const GREEN: &str = "\x1b[38;2;166;227;161m";
    const NC: &str = "\x1b[0m";
    const INFO_ICON: &str = "\u{f05a}"; //
    const CHECK_ICON: &str = "\u{f00c}"; //

    // Path relative to src/keybinds/mod.rs where include_str! is called
    let default_path = "../../keybinds.toml";

    // Try user-specified keybinds file first
    if let Ok(user_keybinds) = std::env::var("USER_KEYBINDS_FILE") {
        let expanded_path = expand_tilde(&user_keybinds);

        if expanded_path.exists() {
            eprintln!(
                "{}{}  {}[keybinds] Using XDG config: {}{}",
                GREEN,
                CHECK_ICON,
                NC,
                BLUE,
                expanded_path.display()
            );
            println!("cargo:rustc-env=KEYBINDS_FILE={}", expanded_path.display());
            println!("cargo:rerun-if-changed={}", expanded_path.display());
            return;
        }
    }

    // Fall back to default keybinds.toml
    eprintln!(
        "{}{}  {}[keybinds] Using default config: keybinds.toml",
        BLUE, INFO_ICON, NC
    );
    println!("cargo:rustc-env=KEYBINDS_FILE={}", default_path);
    println!("cargo:rerun-if-changed=keybinds.toml");
}

/// Expand tilde (~/) in path to HOME directory.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return PathBuf::from(home).join(stripped);
    }
    PathBuf::from(path)
}
