use super::models::{Config, ConfigFile};
use super::scanner::scan_directory;
use std::collections::HashMap;
use std::sync::Arc;

/// Global application state holding the configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    files_by_name: HashMap<String, ConfigFile>,
    allowed_extensions: Vec<String>,
}

impl AppConfig {
    /// Load configuration from file
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path();

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file {}: {}", config_path, e))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;

        // Store allowed extensions
        let allowed_extensions = config.settings.allowed_extensions.clone();

        // Build hashmap for fast lookups
        let mut files_by_name = HashMap::new();

        // Add individual files (no extension validation - config is trusted)
        for file in config.files {
            files_by_name.insert(file.name.clone(), file);
        }

        // Scan directories and add found files
        for dir_config in config.directories {
            match scan_directory(&dir_config) {
                Ok(files) => {
                    for file in files {
                        files_by_name.insert(file.name.clone(), file);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to scan directory {}: {}",
                        dir_config.name, e
                    );
                }
            }
        }

        Ok(AppConfig {
            files_by_name,
            allowed_extensions,
        })
    }

    /// Get all file names
    pub fn list_files(&self) -> Vec<String> {
        let mut names: Vec<_> = self.files_by_name.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get config for a specific file
    pub fn get_file(&self, name: &str) -> Option<&ConfigFile> {
        self.files_by_name.get(name)
    }

    /// Get allowed file extensions
    pub fn allowed_extensions(&self) -> &[String] {
        &self.allowed_extensions
    }

    /// Get the config file path (XDG-compliant)
    ///
    /// Search order:
    /// 1. CONFIG_MANAGER_CONFIG env var
    /// 2. XDG_CONFIG_HOME/config-manager/config.toml
    /// 3. ~/.config/config-manager/config.toml
    /// 4. ./config-manager.toml (fallback)
    fn config_path() -> String {
        use std::path::Path;

        // 1. Explicit override via env var
        if let Ok(path) = std::env::var("CONFIG_MANAGER_CONFIG") {
            return path;
        }

        // 2. XDG_CONFIG_HOME (if set)
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            let path = format!("{}/config-manager/config.toml", xdg_config);
            if Path::new(&path).exists() {
                return path;
            }
        }

        // 3. ~/.config/ (XDG default)
        if let Ok(home) = std::env::var("HOME") {
            let path = format!("{}/.config/config-manager/config.toml", home);
            if Path::new(&path).exists() {
                return path;
            }
        }

        // 4. Fallback: current directory
        "config-manager.toml".to_string()
    }
}

/// Shared application state
pub type SharedConfig = Arc<AppConfig>;
