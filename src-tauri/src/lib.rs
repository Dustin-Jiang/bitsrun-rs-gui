// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use bitsrun::{client::get_login_state, SrunClient};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tauri::State;
use reqwest::Client;

mod api;
mod logout;

use crate::api::RunResult;

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    username: String,
    password: String,
}

struct BitsrunState {
    client: Mutex<Option<SrunClient>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn init_bitsrun(config: UserInfo, state: State<'_, BitsrunState>) -> Result<RunResult, String> {
    println!("init: {} {}", config.username, config.password);
    let client = SrunClient::new(config.username, config.password, None, None, None)
        .await
        .map_err(|e| e.to_string())?;
    *state.client.lock().await = Some(client);
    Ok(RunResult {
        success: true,
        message: "Initialized successfully".to_string(),
    })
}

#[tauri::command]
async fn login(state: State<'_, BitsrunState>) -> Result<RunResult, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().unwrap();
    match client.login(false, true).await {
        Ok(response) => {
            let message = response
                .suc_msg
                .map(|msg| msg.to_string())
                .unwrap_or_else(|| response.error.clone());
            Ok(RunResult {
                success: true,
                message,
            })
        },
        Err(e) => Ok(RunResult {
            success: false,
            message: e.to_string(),
        }),
    }
}

#[tauri::command]
async fn check_login_state() -> Result<RunResult, String> {
    let http_client = Client::new();

    match get_login_state(&http_client, true).await {
        Ok(resp) => Ok(RunResult {
            success: true,
                message: serde_json::to_string(&resp).unwrap(),
            }),
            Err(e) => Ok(RunResult {
                success: false,
                message: e.to_string(),
            }),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(BitsrunState {
            client: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            init_bitsrun,
            login,
            check_login_state,
            logout::logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
