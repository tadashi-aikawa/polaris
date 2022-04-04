#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use chrono::Local;
use parking_lot::RwLock;

use crate::config::Config;
use crate::domain::entity::User;
use crate::external::config;

mod action;
mod domain;
mod external;

struct InnerVigilanciaState {
    config: Config,
    current_user: Option<User>,
}

impl InnerVigilanciaState {
    fn set_current_user(&mut self, user: Option<User>) {
        self.current_user = user;
    }
}

type VigilanciaState = RwLock<InnerVigilanciaState>;

#[tauri::command]
async fn search_messages(
    state: tauri::State<'_, VigilanciaState>,
    query: String,
    exclude_me: bool,
) -> Result<action::search_messages::Response, String> {
    println!(
        "[{}] search_messages: query={}, exclude_me={}",
        Local::now(),
        query,
        exclude_me
    );

    let token = state.read().config.slack_token.clone();
    let my_name = state.read().current_user.clone().map(|x| x.display_name);

    let q = match (my_name, exclude_me) {
        (Some(name), true) => format!("{} -from:@{}", query, name),
        _ => query,
    };

    action::search_messages::exec(&token, q)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn initialize(state: tauri::State<'_, VigilanciaState>) -> Result<(), String> {
    let current_user = tauri::async_runtime::block_on(action::get_current_user::exec(
        state.read().config.slack_token.as_str(),
    ))
        .map(|x| x.user)
        .map_err(|e| e.to_string())?;

    let mut state_guard = state.write();
    state_guard.set_current_user(Some(current_user));

    Ok(())
}

#[tauri::command]
async fn fetch_emoji_list(
    state: tauri::State<'_, VigilanciaState>,
) -> Result<action::get_emoji_list::Response, String> {
    println!("[{}] fetch_emoji_list", Local::now(), );

    let token = state.read().config.slack_token.clone();
    action::get_emoji_list::exec(&token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_all_users(
    state: tauri::State<'_, VigilanciaState>,
) -> Result<action::get_all_users_list::Response, String> {
    println!("[{}] fetch_all_users", Local::now(), );

    let token = state.read().config.slack_token.clone();
    action::get_all_users_list::exec(&token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn fetch_config(state: tauri::State<'_, VigilanciaState>) -> Config {
    state.read().config.clone()
}

fn main() {
    let config = config::load().unwrap();

    tauri::Builder::default()
        .manage(RwLock::new(InnerVigilanciaState {
            config,
            current_user: None,
        }))
        .invoke_handler(tauri::generate_handler![
            initialize,
            search_messages,
            fetch_config,
            fetch_emoji_list,
            fetch_all_users,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
