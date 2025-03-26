import { invoke } from "@tauri-apps/api/core";
import { ApiResp, SrunLoginState } from "./api";
import store from "../store";
import { setProxy } from "./bitsrun";

const getNetStat = async () => {
  // 传递代理设置
  setProxy(store.settings.proxy);
  
  const status = await invoke<ApiResp>("check_login_state");
  if (!status.success) {
    throw status;
  }
  const state = JSON.parse(status.message) as SrunLoginState;
  return new Promise<SrunLoginState>((resolve, reject) => {
    if (state.error === "ok") {
      resolve(state);
    } else {
      reject(status);
    }
  });
};

export {
  getNetStat,
};
