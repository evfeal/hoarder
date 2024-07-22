use std::path::Path;
use chrono::NaiveDate;
use std::env;
use reqwest;
use serde_json::Value;
use regex::Regex;
use url::Url;

pub fn is_video(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        matches!(extension.to_str().unwrap_or(""), "mp4" | "avi" | "mkv" | "mov" | "webm")
    } else {
        false
    }
}

pub fn get_video_metadata(path: &Path) -> Option<(String, NaiveDate)> {
    let filename = path.file_name()?.to_str()?;
    let api_key = match env::var("TMDB_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Failed to get TMDB_API_KEY: {}", e);
            return None;
        }
    };
    
    // Clean up the filename to create a search query
    let query = clean_filename(filename);

    if query.is_empty() {
        eprintln!("Query is empty. Cannot proceed with empty search term.");
        return None;
    }

    let api_url = match Url::parse_with_params(
        "https://api.themoviedb.org/3/search/movie",
        &[("api_key", api_key.as_str()), ("query", &query)]
    ) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to construct URL: {}", e);
            return None;
        }
    };

    let client = reqwest::blocking::Client::new();
    let response = match client.get(api_url).send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
            return None;
        }
    };

    let response_text = match response.text() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to get response text: {}", e);
            return None;
        }
    };

    let json: Value = match serde_json::from_str(&response_text) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return None;
        }
    };

    let first_result = json["results"].get(0)?;
    let title = first_result["title"].as_str()?.to_string();
    let release_date = first_result["release_date"].as_str()?;

    let release_date = match NaiveDate::parse_from_str(release_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Failed to parse release date: {}", e);
            return None;
        }
    };

    let year = release_date.format("%Y").to_string();

    Some((format!("{} ({})", title, year), release_date))
}

fn clean_filename(filename: &str) -> String {
    // Split the filename and extension
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    let (name, _extension) = match parts.as_slice() {
        [ext, name] => (*name, *ext),
        [name] => (*name, ""),
        _ => return String::new(), // Return empty string if filename is empty
    };

    // Remove quality indicators and other common tags
    let re = Regex::new(r"\[[^\]]*\]|\([^\)]*\)|480p|720p|1080p|2160p|4K|UHD|HDR|x264|x265|HEVC|BluRay|WEB-DL|WEBRip").unwrap();
    let cleaned = re.replace_all(name, "");
    
    // Replace dots and underscores with spaces, trim, and remove multiple spaces
    cleaned.replace('.', " ")
           .replace('_', " ")
           .trim()
           .split_whitespace()
           .collect::<Vec<&str>>()
           .join(" ")
}
