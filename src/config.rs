/**
 * Configuration management for blukey
 * @author shashankx86
 * @license MIT
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    #[serde(rename = "DEMON")]
    pub demon: bool,
    #[serde(rename = "KEYS")]
    pub keys: HashMap<String, String>,
}

pub fn get_config_path() -> PathBuf {
    let home_dir = if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        PathBuf::from("/home").join(sudo_user)
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    };
    home_dir.join(".blukey.json")
}

pub fn load_config() -> Config {
    let config_path = get_config_path();
    if !config_path.exists() {
        let default_config = Config {
            demon: true,
            keys: HashMap::new(),
        };
        let json = serde_json::to_string_pretty(&default_config).unwrap();
        fs::write(&config_path, json).unwrap();
        return default_config;
    }
    let contents = fs::read_to_string(&config_path).unwrap();
    match serde_json::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Invalid configuration file: {}", e);
            process::exit(1);
        }
    }
}

pub fn save_config(config: &Config) {
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(get_config_path(), json).unwrap();
}