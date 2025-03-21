use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
  pub success: bool,
  pub message: String,
}