use std::path::{Path, PathBuf};
use std::fs::File;
use exif::{Reader, In, Tag};
use chrono::NaiveDate;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref COUNTERS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn is_image(path: &Path) -> bool {
    if let Ok(buf) = std::fs::read(path) {
        if let Some(kind) = infer::get(&buf) {
            return kind.mime_type().starts_with("image/");
        }
    }
    false
}

pub fn process_image(path: &Path) -> Option<PathBuf> {
    if !is_image(path) {
        return None;
    }

    let date = get_image_date(path);
    
    date.map(|date| rename(path, &date))
}

fn get_image_date(path: &Path) -> Option<NaiveDate> {
    get_exif_date(path).or_else(|| get_fallback_date(path))
}

fn get_exif_date(path: &Path) -> Option<NaiveDate> {
    let file = File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let date_time = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
    let date_str = date_time.value.display_as(date_time.tag).to_string();
    
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").ok()
}

fn get_fallback_date(path: &Path) -> Option<NaiveDate> {
    let filename = path.file_name()?.to_str()?;
    let re = Regex::new(r"(\d{4})(\d{2})(\d{2})").ok()?;
    
    re.find(filename).and_then(|m| {
        let date_str = m.as_str();
        NaiveDate::parse_from_str(date_str, "%Y%m%d").ok()
    })
}

fn rename(path: &Path, date: &NaiveDate) -> PathBuf {
    let parent = path.parent().unwrap_or(Path::new(""));
    let date_str = date.format("%Y%m%d").to_string();
    
    let mut counters = COUNTERS.lock().unwrap();
    let count = counters.entry(date_str.clone()).or_insert(0);
    *count += 1;

    let new_filename = if *count > 1 {
        format!("IMG_{}-{:02}.jpg", date_str, count)
    } else {
        format!("IMG_{}.jpg", date_str)
    };
    
    let new_path = parent.join(new_filename);

    if let Err(e) = std::fs::rename(path, &new_path) {
        eprintln!("Error renaming file: {}", e);
        path.to_path_buf()
    } else {
        println!("Renamed {:?} to {:?}", path, new_path);
        new_path
    }
}
