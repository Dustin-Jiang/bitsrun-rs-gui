use api::get_http_client;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use bitsrun::{client::get_login_state, SrunClient};
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

mod api;
mod settings;

use settings::{Settings, Proxy};

mod login;
mod logout;

use crate::api::RunResult;

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    username: String,
    password: String,
}

struct BitsrunState {
    client: Mutex<Option<SrunClient>>,
    settings: Mutex<Option<Settings>>,
}

#[tauri::command]
async fn init_bitsrun(
    config: UserInfo,
    state: State<'_, BitsrunState>,
) -> Result<RunResult, String> {
    println!("init: {} {}", config.username, config.password);
    let client = SrunClient::new(config.username, config.password, Some(get_http_client(state.clone()).await), None, None)
        .await
        .map_err(|e| e.to_string())?;
    *state.client.lock().await = Some(client);
    Ok(RunResult {
        success: true,
        message: "Initialized successfully".to_string(),
    })
}

#[tauri::command]
async fn set_proxy(state: State<'_, BitsrunState>, proxy: Proxy) -> Result<RunResult, String> {
    println!("set_proxy: received conf: {:?}", proxy);
    let mut guard = state.settings.lock().await;
    let mut settings = guard.as_ref().unwrap().clone();
    settings.proxy = Some(proxy);
    *guard = Some(settings);

    Ok(RunResult {
        success: true,
        message: "Proxy settings updated. ".to_string(),
    })
}

#[tauri::command]
async fn check_login_state(state: State<'_, BitsrunState>) -> Result<RunResult, String> {
    let http_client = api::get_http_client(state).await;

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
            settings: Mutex::new(Some(Settings::new())),
        })
        .invoke_handler(tauri::generate_handler![
            init_bitsrun,
            set_proxy,
            login::login,
            check_login_state,
            logout::logout
        ])
        .on_window_event(|window, event| match event {
          tauri::WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
          }
          _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
