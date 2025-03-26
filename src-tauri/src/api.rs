use serde::{Deserialize, Serialize};
use crate::BitsrunState;
use tauri::State;
use reqwest::{Proxy, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
  pub success: bool,
  pub message: String,
}

pub async fn get_http_client(state: State<'_, BitsrunState>) -> Client {
  let guard = state.settings.lock().await;
  let settings = guard.as_ref().unwrap();

  return settings.proxy.as_ref().map_or_else(
    || Client::builder().build().unwrap(),
    |proxy| {
      let mut builder = Client::builder();
      if proxy.enable {
        if !proxy.system_proxy {
          let http_proxy_scheme = proxy.http_proxy.as_ref().map_or("", |s| s.as_str());
          let https_proxy_scheme = proxy.https_proxy.as_ref().map_or("", |s| s.as_str());

          if !http_proxy_scheme.is_empty() {
            let p = Proxy::http(http_proxy_scheme).unwrap();
            builder = builder.proxy(p);
            println!("proxy: http proxy: {}", http_proxy_scheme);
          }
          if !https_proxy_scheme.is_empty() {
            let p = Proxy::https(https_proxy_scheme).unwrap();
            builder = builder.proxy(p);
            println!("proxy: https proxy: {}", https_proxy_scheme);
          }
        }
        else {
          println!("proxy: use system default proxy");
        }
      } else {
        println!("proxy: disabled");
        builder = builder.no_proxy();
      }
      builder.build().unwrap()
    },
  );
}