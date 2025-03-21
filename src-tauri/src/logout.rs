use crate::api::RunResult;
use crate::BitsrunState;
use tauri::State;

#[tauri::command]
pub async fn logout(state: State<'_, BitsrunState>) -> Result<RunResult, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Client not initialized")?;
    match client.logout(false, false).await {
        Ok(response) => {
            let message = response
                .suc_msg
                .map(|msg| msg.to_string())
                .unwrap_or_else(|| response.error.clone());

            println!("logout: {:?}", message);

            Ok(RunResult { success: true, message })
        },
        Err(e) => {
            eprintln!("logout: {:?}", e);

            Ok(RunResult {
                success: false,
                message: e.to_string(),
            })
        },
    }
}