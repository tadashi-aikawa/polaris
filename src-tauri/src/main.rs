#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod action;
mod external;
mod domain;

use std::sync::Mutex;
use crate::config::Config;
use crate::domain::entity::User;
use crate::external::config;

struct InnerPolarisState {
    config: Config,
    current_user: Option<User>,
}
impl InnerPolarisState {
    fn set_current_user(&mut self, user: Option<User>) {
        self.current_user = user;
    }
}
struct PolarisState(pub Mutex<InnerPolarisState>);

#[tauri::command]
fn search_messages(
    state: tauri::State<'_, PolarisState>,
    query: String,
) -> Result<action::search_messages::Response, String> {
    println!("search_messages");

    tauri::async_runtime::block_on(
        action::search_messages::exec(
            state.0.lock().unwrap().config.slack_token.as_str(), query
        )
    )
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn initialize(
    state: tauri::State<'_, PolarisState>,
) -> Result<(), String> {
    let current_user  = tauri::async_runtime::block_on(
        action::get_current_user::exec(
            state.0.lock().unwrap().config.slack_token.as_str()
        )
    )
        .map(|x| x.user)
        .map_err(|e| e.to_string())?;

    let mut state_guard = state.0.lock().unwrap();
    state_guard.set_current_user(Some(current_user));

    Ok(())
}

#[tauri::command]
fn fetch_queries(
    state: tauri::State<'_, PolarisState>,
) -> Vec<String> {
    state.0.lock().unwrap().config.queries.to_vec()
}

fn main() {
    let config = config::load().unwrap();

    tauri::Builder::default()
        .manage(PolarisState(Mutex::new(InnerPolarisState { config, current_user: None })))
        .invoke_handler(tauri::generate_handler![initialize, search_messages, fetch_queries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
