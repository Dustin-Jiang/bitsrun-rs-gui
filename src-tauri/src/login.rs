use crate::api::RunResult;
use crate::BitsrunState;
use tauri::State;

#[tauri::command]
pub async fn login(state: State<'_, BitsrunState>) -> Result<RunResult, String> {
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
        }
        Err(e) => Ok(RunResult {
            success: false,
            message: e.to_string(),
        }),
    }
}