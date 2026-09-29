use std::fs;
use std::net::{SocketAddr,IpAddr,Ipv4Addr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ConfigData {
    pub bind_address: SocketAddr,
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
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 8080),
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
        log::warn!("No config file found at '{path}'. Using default values!");
        return ConfigData::default();
    }
    
    log::debug!("Reading config file...");
    let contents = fs::read_to_string(path)
        .expect("Unable to read config file!");

    log::debug!("Pasing configuration...");
    let config_data: ConfigData = serde_json::from_str(&contents)
        .unwrap_or_default();
    
    log::debug!("Configuration loaded!");
    return config_data;
}
