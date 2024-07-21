use std::path::{Path, PathBuf};
use std::fs;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::image;

lazy_static! {
    static ref COUNTERS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn organize_files(path: &str) {
    let path = Path::new(path);
    if path.is_dir() {
        process_directory(path);
    } else {
        process_single_file(path);
    }
}

fn process_directory(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                process_single_file(&path);
            } else if path.is_dir() {
                process_directory(&path);
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", dir);
    }
}

fn process_single_file(path: &Path) {
    if image::is_image(path) {
        organize_image(path);
    } else {
        // Here you can add more file type checks and organizing methods
        println!("Skipping non-image file: {:?}", path);
    }
}

fn organize_image(path: &Path) {
    if let Some(date) = image::get_image_date(path) {
        let new_path = create_new_path(path, &date);
        move_file(path, &new_path);
    }
}

fn create_new_path(path: &Path, date: &NaiveDate) -> PathBuf {
    let year_dir = path.parent().unwrap().join(date.format("%Y").to_string());
    fs::create_dir_all(&year_dir).unwrap_or_else(|e| eprintln!("Error creating directory: {}", e));

    let date_str = date.format("%m%d").to_string();
    
    let mut counters = COUNTERS.lock().unwrap();
    let count = counters.entry(date_str.clone()).or_insert(0);
    *count += 1;

    let new_filename = if *count > 1 {
        format!("{}-{:02}.jpg", date_str, count)
    } else {
        format!("{}.jpg", date_str)
    };

    year_dir.join(new_filename)
}

fn move_file(old_path: &Path, new_path: &PathBuf) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error moving file: {}", e);
    } else {
        println!("Moved {:?} to {:?}", old_path, new_path);
    }
}
