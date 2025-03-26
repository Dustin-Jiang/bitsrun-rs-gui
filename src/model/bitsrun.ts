import { invoke } from "@tauri-apps/api/core";
import store, { NetworkStatus } from "../store";
import { ApiResp } from "./api";

const init = async (username: string, password: string) => {
  // 传递代理设置
  await invoke("set_proxy", { proxy: store.settings.proxy });

  await invoke("init_bitsrun", { config: {
    username: username,
    password: password
  } });
  store.status = NetworkStatus.INIT;
}

const login = async (username: string, password: string) => {
  // 初始化 bitsrun
  await init(username, password);

  // 登录
  store.status = NetworkStatus.PENDING;
  const resp = await invoke("login") as ApiResp;
  if (resp.message === "login_error") {
    store.status = NetworkStatus.OFFLINE;
  } else {
    store.status = NetworkStatus.ONLINE;
  }
}

export { init, login };