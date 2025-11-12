use std::sync::RwLock;

use once_cell::sync::Lazy;

struct Config {
    ttl: u64,
    geocode_url: String,
    forecast_url: String,
}

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    RwLock::new(Config {
        ttl: 0,
        geocode_url: "".into(),
        forecast_url: "".into(),
    })
});

#[tarantool::proc]
pub fn update_config(ttl: u64, geocode_url: String, forcast_url: String) {
    let mut cfg = CONFIG
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    cfg.ttl = ttl;
    cfg.forecast_url = forcast_url;
    cfg.geocode_url = geocode_url;
}

pub fn get_ttl() -> u64 {
    CONFIG
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .ttl
}

pub fn get_geocode_url() -> String {
    CONFIG
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .geocode_url
        .clone()
}

pub fn get_forecast_url() -> String {
    CONFIG
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .forecast_url
        .clone()
}
