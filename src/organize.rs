use crate::image;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static COUNTERS: Lazy<Mutex<HashMap<String, usize>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn organize_files(path: &PathBuf) -> Result<()> {
    if path.is_dir() {
        process_directory(path)
    } else {
        process_single_file(path)
    }
}

fn process_directory(dir: &Path) -> Result<()> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .try_for_each(|entry| -> Result<()> {
            let entry = entry?;
            if entry.file_type().is_file() {
                process_single_file(&entry.path())?;
            }
            Ok(())
        })
}

fn process_single_file(path: &Path) -> Result<()> {
    if image::is_image(path) {
        organize_image(path)?;
    } else {
        println!("Skipping unsupported file type: {:?}", path);
    }
    Ok(())
}

fn organize_image(path: &Path) -> Result<()> {
    if let Some(date) = image::get_image_date(path) {
        let new_path = create_new_path(path, &date)?;
        move_file(path, &new_path)?;
    }
    Ok(())
}

fn create_new_path(path: &Path, date: &NaiveDate) -> Result<PathBuf> {
    let parent = path.parent().unwrap_or(Path::new(""));
    let year_dir = parent.join(date.format("%Y").to_string());
    
    fs::create_dir_all(&year_dir)
        .with_context(|| format!("Failed to create directory: {:?}", year_dir))?;

    let date_str = date.format("%Y-%m-%d").to_string();

    let mut counters = COUNTERS.lock().unwrap();
    let count = counters.entry(date_str.clone()).or_insert(0);
    *count += 1;

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("jpg");
    let new_filename = if *count > 1 {
        format!("{}-{:02}.{}", date_str, count, extension)
    } else {
        format!("{}.{}", date_str, extension)
    };

    Ok(year_dir.join(new_filename))
}

fn move_file(old_path: &Path, new_path: &PathBuf) -> Result<()> {
    fs::rename(old_path, new_path)
        .with_context(|| format!("Failed to move file from {:?} to {:?}", old_path, new_path))?;
    println!("Moved {:?} to {:?}", old_path, new_path);
    Ok(())
}
