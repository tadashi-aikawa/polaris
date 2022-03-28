#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod action;
mod external;
mod domain;

use crate::config::Config;
use crate::external::config;

struct PolarisState {
    config: Config,
}

#[tauri::command]
async fn search_messages(
    state: tauri::State<'_, PolarisState>,
    query: String,
) -> Result<action::search_messages::Response, String> {
    action::search_messages::exec(state.config.slack_token.as_str(), query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_current_user(
    state: tauri::State<'_, PolarisState>,
) -> Result<action::get_current_user::Response, String> {
    action::get_current_user::exec(state.config.slack_token.as_str())
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    let config = config::load().unwrap();

    tauri::Builder::default()
        .manage(PolarisState { config })
        .invoke_handler(tauri::generate_handler![search_messages, get_current_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
