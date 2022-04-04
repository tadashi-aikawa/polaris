use std::fs::File;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::api::path::home_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub slack_token: String,
    pub interval_sec: i32,
    pub conditions: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub title: Option<String>,
    pub color: Option<String>,
    pub query: String,
    pub interval_sec: Option<i32>,
    pub should_notify: Option<bool>,
    pub include_me: Option<bool>,
}

pub fn load() -> Result<Config> {
    let dir = home_dir().ok_or_else(|| anyhow!("Can't find home_dir!"))?;
    let f = File::open(dir.join(".vigilancia.json"))?;
    let r: Config = serde_json::from_reader(f).unwrap();
    Ok(r)
}
