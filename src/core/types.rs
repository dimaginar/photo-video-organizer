use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Photo,
    Video,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhotoFile {
    pub path: PathBuf,
    pub date_taken: DateTime<Utc>,
    pub file_type: FileType,
    pub hash: Option<String>, 
}

#[derive(Debug, Clone)]
pub struct OrganizeSettings {
    pub target_dir: PathBuf,
    pub dry_run: bool, 
}

#[derive(Debug, Default, Clone)]
pub struct OrganizationResult {
    pub processed_files: usize,
    pub moved_files: usize,
    pub photos_moved: usize,
    pub videos_moved: usize,
    pub duplicates_found: usize,
    pub photos_per_year: BTreeMap<String, usize>,
    pub videos_per_year: BTreeMap<String, usize>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

use std::fs;
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub last_source_dir: Option<String>,
    pub last_target_dir: Option<String>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
}

impl AppConfig {
    fn get_config_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "PhotoSort") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                let _ = fs::create_dir_all(config_dir);
            }
            Some(config_dir.join("config.json"))
        } else {
            None
        }
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::get_config_path() {
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, content);
            }
        }
    }
}
