use crate::{image, video};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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
    } else if video::is_video(path) {
        organize_video(path);
    } else {
        println!("Skipping unsupported file type: {:?}", path);
    }
}

fn organize_image(path: &Path) {
    if let Some(date) = image::get_image_date(path) {
        let new_path = create_new_path(path, &date);
        move_file(path, &new_path);
    }
}

fn organize_video(path: &Path) {
    if let Some((title, release_date)) = video::get_video_metadata(path) {
        let new_path = create_new_video_path(path, &title, &release_date);
        create_movie_folder(&new_path);
        move_file(path, &new_path);
    }
}

fn create_new_path(path: &Path, date: &NaiveDate) -> PathBuf {
    let parent = path.parent().unwrap_or(Path::new(""));
    let date_str = date.format("%Y-%m-%d").to_string();

    let mut counters = COUNTERS.lock().unwrap();
    let count = counters.entry(date_str.clone()).or_insert(0);
    *count += 1;

    let new_filename = if *count > 1 {
        format!("{}-{:02}.jpg", date_str, count)
    } else {
        format!("{}.jpg", date_str)
    };

    parent.join(new_filename)
}

fn create_new_video_path(path: &Path, title: &str, date: &NaiveDate) -> PathBuf {
    let year_dir = path
        .parent()
        .unwrap_or(Path::new(""))
        .join(date.format("%Y").to_string());
    fs::create_dir_all(&year_dir).unwrap_or_else(|e| eprintln!("Error creating directory: {}", e));

    let formatted_title = title
        .replace(' ', ".")
        .replace(|c: char| !c.is_alphanumeric() && c != '.', "");
    let safe_title = format!("{}.({})", formatted_title, date.format("%Y"));

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("mp4");

    let movie_folder = year_dir.join(&safe_title);
    let new_filename = format!("{}.{}", safe_title, extension);

    movie_folder.join(new_filename)
}

fn create_movie_folder(new_path: &Path) {
    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|e| eprintln!("Error creating movie folder: {}", e));
    }
}

fn move_file(old_path: &Path, new_path: &PathBuf) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error moving file: {}", e);
    } else {
        println!("Moved {:?} to {:?}", old_path, new_path);
    }
}
