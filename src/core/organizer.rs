use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read};
use anyhow::{Result, bail, Context};
use sha2::{Sha256, Digest};
use chrono::Datelike;
use log::{info, warn};

use crate::core::types::{PhotoFile, FileType, OrganizationResult, OrganizeSettings};

pub fn validate_directories(source: &Path, target: &Path) -> Result<()> {
    if !source.exists() {
        bail!("Source directory does not exist: {:?}", source);
    }
    
    let source_canon = source.canonicalize().context("Failed to resolve source path")?;
    
    if target.exists() {
        let target_canon = target.canonicalize().context("Failed to resolve target path")?;
        
        if source_canon == target_canon {
            bail!("Source and target cannot be the same directory");
        }
        
        if target_canon.starts_with(&source_canon) {
            bail!("Target cannot be inside source directory");
        }
        
        if source_canon.starts_with(&target_canon) {
            bail!("Source cannot be inside target directory");
        }
    }

    Ok(())
}

pub fn create_target_structure(target: &Path) -> Result<()> {
    fs::create_dir_all(target.join("Photos"))?;
    fs::create_dir_all(target.join("Videos"))?;
    fs::create_dir_all(target.join("Duplicates"))?;
    Ok(())
}

fn calculate_file_hash(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; 

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn get_unique_target_path(base_dir: &Path, original_path: &Path) -> PathBuf {
    let file_name = original_path.file_name().unwrap_or_default();
    let mut target_path = base_dir.join(file_name);
    
    let file_stem = original_path.file_stem().unwrap().to_string_lossy();
    let extension = original_path.extension().unwrap_or_default().to_string_lossy();
    let ext_dot = if extension.is_empty() { "" } else { "." };

    let mut counter = 1;
    while target_path.exists() {
        let new_name = format!("{}_copy_{}{}{}", file_stem, counter, ext_dot, extension);
        target_path = base_dir.join(new_name);
        counter += 1;
    }
    
    target_path
}

pub fn organize_files(files: Vec<PhotoFile>, settings: &OrganizeSettings) -> OrganizationResult {
    let mut result = OrganizationResult::default();
    
    for file in files {
        let mut file = file; // Allow move
        result.processed_files += 1;

        let year = file.date_taken.year();
        let year_folder = year.to_string();
        
        let category_folder = match file.file_type {
            FileType::Photo => "Photos",
            FileType::Video => "Videos",
        };
        
        let target_year_dir = settings.target_dir.join(category_folder).join(&year_folder);
        
        if !settings.dry_run && !target_year_dir.exists() {
             if let Err(e) = fs::create_dir_all(&target_year_dir) {
                 result.errors.push(format!("Failed to create dir {:?}: {}", target_year_dir, e));
                 continue;
             }
        }
        
        let file_name = file.path.file_name().unwrap();
        let standard_target_path = target_year_dir.join(file_name);
        
        let final_dest: PathBuf;
        let mut is_duplicate = false;

        if standard_target_path.exists() {
            let src_hash = match calculate_file_hash(&file.path) {
                Ok(h) => h,
                Err(e) => {
                    result.errors.push(format!("Failed to hash source {:?}: {}", file.path, e));
                    continue;
                }
            };
            file.hash = Some(src_hash.clone());
            
            let dst_hash = match calculate_file_hash(&standard_target_path) {
                Ok(h) => h,
                Err(e) => {
                     warn!("Could not hash existing file {:?} ({}), treating as name collision.", standard_target_path, e);
                     "UNKNOWN".to_string()
                }
            };
            
            if src_hash == dst_hash {
                is_duplicate = true;
                final_dest = get_unique_target_path(&settings.target_dir.join("Duplicates"), &file.path);
            } else {
                final_dest = get_unique_target_path(&target_year_dir, &file.path);
            }
        } else {
            final_dest = standard_target_path;
        }
        
        if !settings.dry_run {
            if let Some(parent) = final_dest.parent() {
                if !parent.exists() {
                    let _ = fs::create_dir_all(parent);
                }
            }

            if let Err(e) = fs::rename(&file.path, &final_dest) {
                 if let Err(copy_err) = fs::copy(&file.path, &final_dest) {
                      result.errors.push(format!("Failed to move {:?} to {:?}: {} (Copy also failed: {})", file.path, final_dest, e, copy_err));
                      continue;
                 } else {
                     if let Err(del_err) = fs::remove_file(&file.path) {
                          result.warnings.push(format!("Copied but failed to delete source {:?}: {}", file.path, del_err));
                     }
                 }
            }
        } else {
            info!("[DRY RUN] Move {:?} -> {:?}", file.path, final_dest);
        }

        if is_duplicate {
            result.duplicates_found += 1;
        } else {
            result.moved_files += 1;
            match file.file_type {
                FileType::Photo => {
                    result.photos_moved += 1;
                    *result.photos_per_year.entry(year_folder).or_insert(0) += 1;
                },
                FileType::Video => {
                    result.videos_moved += 1;
                    *result.videos_per_year.entry(year_folder).or_insert(0) += 1;
                }
            }
        }
    }

    result
}

