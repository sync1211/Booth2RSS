use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ConfigData {
    pub currency_fallback: String,
    pub cache_minutes: u64,
    pub cache_size: u64,
    pub allow_currency_conversion: bool,
    pub currency_conversion_cache_size: u64,
    pub currency_conversion_cache_ttl_minutes: u64
}

impl Default for ConfigData {
    fn default() -> Self {
        return ConfigData {
            currency_fallback: "JPY".to_string(),
            cache_minutes: 15,
            cache_size: 50,
            allow_currency_conversion: true,
            currency_conversion_cache_size: 10,
            currency_conversion_cache_ttl_minutes: 120
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
