use std::path::Path;
use glob::glob;
use std::fs;
use crate::image;

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
    for entry in glob(pattern).expect("Failed to read glob (im sorry)") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    if let Some(_new_path) = image::process_image(&path) {
                        // Image was processed and renamed
                        continue;
                    }
                    rename_single_file(&path, prefix, suffix);
                } else if path.is_dir() {
                    rename_directory_files(&path, prefix, suffix);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn rename_single_file(path: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let new_name = get_new_name(name, prefix, suffix);
        let new_path = path.with_file_name(new_name);
        perform_rename(path, &new_path);
    }
}

fn rename_directory_files(dir: &Path, prefix: Option<&String>, suffix: Option<&String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(_new_path) = image::process_image(&path) {
                    // Image was processed and renamed
                    continue;
                }
                rename_single_file(&path, prefix, suffix);
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", dir);
    }
}

fn get_new_name(name: &str, prefix: Option<&String>, suffix: Option<&String>) -> String {
    match (prefix, suffix) {
        (Some(p), None) => format!("{}{}", p, name),
        (None, Some(s)) => {
            let (name_without_ext, ext) = name.rsplit_once('.').unwrap_or((name, ""));
            if ext.is_empty() {
                format!("{}{}", name, s)
            } else {
                format!("{}{}.{}", name_without_ext, s, ext)
            }
        },
        (Some(p), Some(s)) => {
            let (name_without_ext, ext) = name.rsplit_once('.').unwrap_or((name, ""));
            if ext.is_empty() {
                format!("{}{}{}", p, name, s)
            } else {
                format!("{}{}{}.{}", p, name_without_ext, s, ext)
            }
        },
        _ => name.to_string(),
    }
}

fn perform_rename(old_path: &Path, new_path: &Path) {
    if let Err(e) = fs::rename(old_path, new_path) {
        eprintln!("Error renaming {:?}: {}", old_path, e);
    } else {
        println!("Succesfully renamed {:?} to {:?}", old_path, new_path);
    }
}
