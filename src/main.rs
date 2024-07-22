mod api_config;
mod image;
mod organize;
mod simple;
mod video;

use api_config::APIConfig;
use clap::{Arg, Command};
use std::env;

fn main() {
    let mut config = APIConfig::load();

    if let Ok(api_key) = env::var("TMDB_API_KEY") {
        config.tmdb_api_key = Some(api_key);
    }

    if config.tmdb_api_key.is_none() {
        println!("Please enter your TMDB API key:");
        let mut api_key = String::new();
        std::io::stdin().read_line(&mut api_key).unwrap();
        config.tmdb_api_key = Some(api_key.trim().to_string());
        config.save().unwrap();
    }

    if let Some(api_key) = &config.tmdb_api_key {
        env::set_var("TMDB_API_KEY", api_key);
    }

    let matches = Command::new("Hoarder")
        .version("1.0")
        .author("Evan Alvarez")
        .about("The all-in-one renaming tool for people with too many files")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Path to the file(s) or directories to be renamed")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("prefix")
                .short('p')
                .long("prefix")
                .value_name("PREFIX")
                .help("Prefix to add to filenames")
                .conflicts_with("suffix"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .value_name("SUFFIX")
                .help("Suffix to add to filenames"),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .help("Organize files into a directory based structure")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let paths: Vec<&String> = matches.get_many("paths").unwrap().collect();
    let prefix = matches.get_one::<String>("prefix");
    let suffix = matches.get_one::<String>("suffix");
    let use_directory_structure = matches.get_flag("directory");

    for path in paths {
        if use_directory_structure {
            organize::organize_files(path);
        } else {
            process_path(path, prefix, suffix);
        }
    }
}

fn process_path(path: &str, prefix: Option<&String>, suffix: Option<&String>) {
    let path = std::path::Path::new(path);
    if path.is_dir() {
        for entry in std::fs::read_dir(path).expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    process_file(&entry_path, prefix, suffix);
                }
            }
        }
    } else {
        process_file(path, prefix, suffix);
    }
}

fn process_file(path: &std::path::Path, prefix: Option<&String>, suffix: Option<&String>) {
    if video::is_video(path) {
        if let Some((title, _release_date)) = video::get_video_metadata(path) {
            let new_filename = format!("{}.mp4", title);
            let new_path = path.with_file_name(new_filename);
            if let Err(e) = std::fs::rename(path, &new_path) {
                eprintln!("Error renaming video file: {}", e);
            } else {
                println!("Renamed video: {:?} to {:?}", path, new_path);
            }
        }
    } else if image::is_image(path) {
        if let Some(new_path) = image::process_image(path) {
            println!("Processed image: {:?} to {:?}", path, new_path);
        }
    } else {
        simple::rename_single_file(path, prefix, suffix);
    }
}
