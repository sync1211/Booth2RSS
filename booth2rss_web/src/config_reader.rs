use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ConfigData {
    pub currency_source: String,
    pub cache_minutes: u64,
    pub cache_size: u64
}

impl Default for ConfigData {
    fn default() -> Self {
        return ConfigData {
            currency_source: "JPY".to_string(),
            cache_minutes: 15,
            cache_size: 50
        }
    }
}

pub fn read_config(path: &str) -> ConfigData {
    if !fs::exists(path).unwrap_or(false) {
        eprintln!("No config file found at {}", path);
        return ConfigData::default();
    }

    let contents = fs::read_to_string(path)
        .expect("Unable to read config file!");

    let config_data: ConfigData = serde_json::from_str(&contents)
        .unwrap_or_default();
    
    return config_data;
}
