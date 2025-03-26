import { invoke } from "@tauri-apps/api/core";
import { ApiResp, SrunLoginState } from "./api";
import store from "../store";

const getNetStat = async () => {
  // 传递代理设置
  await invoke("set_proxy", { proxy: store.settings.proxy});
  
  const status = await invoke<ApiResp>("check_login_state");
  if (!status.success) {
    throw new Error(status.message);
  }
  const state = JSON.parse(status.message) as SrunLoginState;
  return new Promise<SrunLoginState>((resolve, reject) => {
    if (state.error === "ok") {
      resolve(state);
    } else {
      reject(state);
    }
  });
};

export {
  getNetStat,
};
