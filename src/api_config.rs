use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct APIConfig {
    pub tmdb_api_key: Option<String>,
}

impl APIConfig {
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string("config.toml") {
            toml::from_str(&content).unwrap_or_default()
        } else {
            APIConfig::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let content = toml::to_string(self).unwrap();
        fs::write("config.toml", content)
    }
}

impl Default for APIConfig {
    fn default() -> Self {
        APIConfig { tmdb_api_key: None }
    }
}
