use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub readonly: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigDirectory {
    pub path: String,
    pub name: String,
    #[serde(default = "default_depth")]
    pub depth: usize,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub readonly: bool,
}

fn default_depth() -> usize {
    3
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub files: Vec<ConfigFile>,
    #[serde(default)]
    pub directories: Vec<ConfigDirectory>,
}

/// Global application state holding the configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    files_by_name: HashMap<String, ConfigFile>,
}

impl AppConfig {
    /// Scan a directory and return all matching files
    fn scan_directory(dir_config: &ConfigDirectory) -> Result<Vec<ConfigFile>, String> {
        let mut found_files = Vec::new();
        let base_path = Path::new(&dir_config.path);

        // Expand home directory
        let expanded_path = if dir_config.path.starts_with("~/") {
            let home = std::env::var("HOME")
                .map_err(|_| "HOME environment variable not set".to_string())?;
            PathBuf::from(home).join(&dir_config.path[2..])
        } else {
            base_path.to_path_buf()
        };

        if !expanded_path.exists() {
            return Err(format!(
                "Directory does not exist: {}",
                expanded_path.display()
            ));
        }

        // Walk directory with depth limit
        for entry in WalkDir::new(&expanded_path)
            .max_depth(dir_config.depth)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();

            // Check file extension matches allowed types
            if !dir_config.types.is_empty() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if !dir_config.types.iter().any(|t| t == ext) {
                        continue;
                    }
                } else {
                    continue; // No extension, skip
                }
            }

            // Create ConfigFile entry
            let relative_path = path
                .strip_prefix(&expanded_path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Use directory name as prefix for uniqueness
            let display_name = if relative_path.contains('/') || relative_path.contains('\\') {
                format!("{}/{}", dir_config.name, relative_path)
            } else {
                format!("{}/{}", dir_config.name, file_name)
            };

            found_files.push(ConfigFile {
                path: path.to_string_lossy().to_string(),
                name: display_name,
                description: format!("From directory: {}", dir_config.description),
                readonly: dir_config.readonly,
            });
        }

        // Sort by path for consistent ordering
        found_files.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(found_files)
    }

    /// Load configuration from file
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path();

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file {}: {}", config_path, e))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;

        // Build hashmap for fast lookups
        let mut files_by_name = HashMap::new();

        // Add individual files
        for file in config.files {
            // Validate name ends with .conf or .toml
            if !file.name.ends_with(".conf") && !file.name.ends_with(".toml") {
                return Err(format!(
                    "File name must end with .conf or .toml: {}",
                    file.name
                ));
            }
            files_by_name.insert(file.name.clone(), file);
        }

        // Scan directories and add found files
        for dir_config in config.directories {
            match Self::scan_directory(&dir_config) {
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

        Ok(AppConfig { files_by_name })
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

    /// Get the config file path
    fn config_path() -> String {
        std::env::var("CONFIG_MANAGER_CONFIG").unwrap_or_else(|_| "config-manager.toml".to_string())
    }
}

/// Shared application state
pub type SharedConfig = Arc<AppConfig>;
