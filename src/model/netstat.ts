import { invoke } from "@tauri-apps/api/core";
import { ApiResp, SrunLoginState } from "./api";

const getNetStat = async () => {
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
