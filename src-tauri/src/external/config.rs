use std::fs::File;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::api::path::home_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub slack_token: String,
}

pub fn load() -> Result<Config> {
    let dir = home_dir().ok_or_else(|| anyhow!("Can't find home_dir!"))?;
    let f = File::open(dir.join(".polaris.json")).unwrap();
    let r: Config = serde_json::from_reader(f).unwrap();
    Ok(r)
}
