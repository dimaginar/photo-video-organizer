use std::path::Path;
use walkdir::WalkDir;
use anyhow::Result;
use log::error;

use crate::core::types::{PhotoFile, FileType};
use crate::core::date_utils::{extract_photo_date, extract_video_date};

pub fn is_photo(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "heic")
}

pub fn is_video(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), "mp4" | "mov" | "avi")
}

pub fn scan_directory(source: &Path) -> Result<Vec<PhotoFile>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(source).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let file_type = if is_photo(ext) {
                    Some(FileType::Photo)
                } else if is_video(ext) {
                    Some(FileType::Video)
                } else {
                    None
                };

                if let Some(ft) = file_type {
                    let date_extract_res = match ft {
                        FileType::Photo => extract_photo_date(path),
                        FileType::Video => extract_video_date(path),
                    };

                    match date_extract_res {
                        Ok((date_taken, _warn)) => {
                             files.push(PhotoFile {
                                 path: path.to_path_buf(),
                                 date_taken,
                                 file_type: ft,
                                 hash: None, 
                             });
                        },
                        Err(e) => {
                            error!("Failed to extract date for {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    }
    
    Ok(files)
}
