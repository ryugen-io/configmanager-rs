// Server version info
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version_string() -> String {
    format!("config-manager-server v{}", SERVER_VERSION)
}
