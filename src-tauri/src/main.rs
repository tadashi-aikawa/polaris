#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod external;

use crate::config::Config;
use crate::external::config;

struct PolarisState {
    config: Config,
}

#[tauri::command]
async fn search_messages(
    state: tauri::State<'_, PolarisState>,
    query: String,
) -> Result<command::search_messages::Response, String> {
    command::search_messages::exec(state.config.slack_token.as_str(), query)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    let config = config::load().unwrap();

    tauri::Builder::default()
        .manage(PolarisState { config })
        .invoke_handler(tauri::generate_handler![search_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
