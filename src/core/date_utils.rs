use std::fs;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use exif::{In, Reader, Tag, Exif};
use anyhow::Result;
use log::warn;

fn parse_exif_datetime(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S") {
        return Some(Utc.from_utc_datetime(&naive));
    }
    None
}

pub fn get_file_modification_date(path: &Path) -> Result<DateTime<Utc>> {
    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;
    Ok(DateTime::from(modified))
}

pub fn extract_photo_date(path: &Path) -> Result<(DateTime<Utc>, bool)> {
    let file_res = fs::File::open(path);
    if let Ok(file) = file_res {
        let mut bufreader = std::io::BufReader::new(&file);
        let reader = Reader::new();
        
        if let Ok(exif) = reader.read_from_container(&mut bufreader) {
            let exif: Exif = exif;
            // Priority 1: DateTimeOriginal
            if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                let s = field.display_value().to_string();
                if !s.is_empty() {
                     if let Some(dt) = parse_exif_datetime(&s) {
                        return Ok((dt, false));
                    }
                }
            }

            // Priority 2: DateTime
             if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                let s = field.display_value().to_string();
                if !s.is_empty() {
                     if let Some(dt) = parse_exif_datetime(&s) {
                        return Ok((dt, true)); 
                    }
                }
            }
            
            // Priority 3: DateTimeDigitized
             if let Some(field) = exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
                let s = field.display_value().to_string();
                if !s.is_empty() {
                     if let Some(dt) = parse_exif_datetime(&s) {
                        return Ok((dt, true)); 
                    }
                }
            }
        }
    }
    
    let mod_date = get_file_modification_date(path)?;
    warn!("No EXIF data found for {:?}, using file modification date: {}", path, mod_date);
    Ok((mod_date, true))
}

pub fn extract_video_date(path: &Path) -> Result<(DateTime<Utc>, bool)> {
    let mod_date = get_file_modification_date(path)?;
    warn!("Using file modification date for video (MVP): {:?} -> {}", path, mod_date);
    Ok((mod_date, true))
}
