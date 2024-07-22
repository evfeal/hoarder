use std::path::{Path, PathBuf};
use glob::glob;
use std::fs;
use chrono::NaiveDate;
use crate::{image, video};

pub fn rename_files(pattern: &str, prefix: Option<&String>, suffix: Option<&String>) {
    let path = Path::new(pattern);
    if path.is_dir() {
        let glob_pattern = format!("{}/*", pattern);
        process_glob(&glob_pattern, prefix, suffix);
    } else {
        process_glob(pattern, prefix, suffix);
    }
}

fn process_glob(pattern: &str, prefix: Option<&String>, suffix: Option<&String>) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    if video::is_video(&path) {
                        rename_video_file(&path);
                    } else if let Some(_new_path) = image::process_image(&path) {
                        // Image was processed and renamed
                        continue;
                    } else {
                        rename_single_file(&path, prefix, suffix);
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn rename_video_file(path: &Path) {
    if let Some((title, release_date)) = video::get_video_metadata(path) {
        let new_name = format_video_filename(&title, &release_date, path);
        let new_path = path.with_file_name(new_name);
        perform_rename(path, &new_path);
    }
}

fn format_video_filename(title: &str, date: &NaiveDate, path: &Path) -> String {
    let formatted_title = title.replace(' ', ".")
                               .replace(|c: char| !c.is_alphanumeric() && c != '.', "");
    let safe_title = format!("{}.({})", formatted_title, date.format("%Y"));
    let extension = path.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("mp4");
    format!("{}.{}", safe_title, extension)
}

pub fn rename_single_file(path: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let new_name = get_new_name(name, prefix, suffix);
        let new_path = path.with_file_name(new_name);
        perform_rename(path, &new_path);
    }
}

fn get_new_name(name: &str, prefix: Option<&String>, suffix: Option<&String>) -> String {
    let (name_without_ext, ext) = name.rsplit_once('.').unwrap_or((name, ""));
    
    // Replace spaces with dots in the name
    let name_without_ext = name_without_ext.replace(' ', ".");
    
    let new_name = match (prefix, suffix) {
        (Some(p), Some(s)) => format!("{}{}{}", p, name_without_ext, s),
        (Some(p), None) => format!("{}{}", p, name_without_ext),
        (None, Some(s)) => format!("{}{}", name_without_ext, s),
        (None, None) => name_without_ext,
    };
    
    if ext.is_empty() {
        new_name
    } else {
        format!("{}.{}", new_name, ext)
    }
}

fn perform_rename(old_path: &Path, new_path: &Path) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error renaming {:?}: {}", old_path, e);
    } else {
        println!("Successfully renamed {:?} to {:?}", old_path, new_path);
    }
}
