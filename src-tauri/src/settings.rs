use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub proxy: Option<Proxy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proxy {
  pub enable: bool,
  pub system_proxy: bool,
  pub http_proxy: Option<String>,
  pub https_proxy: Option<String>,
}

impl Proxy {
  pub fn new() -> Self {
    Self {
      enable: false,
      system_proxy: true,
      http_proxy: None,
      https_proxy: None,
    }
  }
}

impl Settings {
  pub fn new() -> Self {
    Self { proxy: Some(Proxy::new()) }
  }
}