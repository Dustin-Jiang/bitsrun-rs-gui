import { invoke } from "@tauri-apps/api/core";
import store, { NetworkStatus } from "../store";

interface ApiResp<T = string> {
  success: boolean;
  message: T;
}

interface SrunLoginState {
  error: "ok" | "not_online_error" | unknown;
  online_ip: string;

  // present when logged in
  server_flag?: number;
  add_time?: number;
  all_bytes?: number;
  bytes_in?: number;
  bytes_out?: number;
  checkout_date?: number;
  domain?: string;
  group_id?: string;
  keepalive_time?: number;
  products_name?: string;
  real_name?: string;
  remain_bytes?: number;
  remain_seconds?: number;
  sum_bytes?: number;
  sum_seconds?: number;
  sysver?: string;
  user_balance?: number;
  user_charge?: number;
  user_mac?: string;
  user_name?: string;
  wallet_balance?: number;

  // present when logged out
  client_ip?: string;
  error_msg?: string;
  res?: string;
  srun_ver?: string;
  st?: number;
}

const init = async () => {
  await invoke("init_bitsrun", {
    config: {
      username: store.userInfo.username,
      password: store.userInfo.password
    }
  });
  store.status = NetworkStatus.INIT;
}

export {
  init,
  type SrunLoginState,
  type ApiResp,
};