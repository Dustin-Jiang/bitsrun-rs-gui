import { invoke } from "@tauri-apps/api/core";
import store, { NetworkStatus } from "../store";
import { ApiResp } from "./api";
import { ProxySettings } from "../store/settings";

const setProxy = async (proxy: ProxySettings) => {
  await invoke("set_proxy", { proxy });
}

const init = async (username: string, password: string) => {
  // 传递代理设置
  setProxy(store.settings.proxy);

  try {
    await invoke("init_bitsrun", { config: {
      username: username,
      password: password
    } });
  }
  catch(e) {
    throw e;
  }
  store.status = NetworkStatus.INIT;
}

const login = async (username: string, password: string) => {
  // 初始化 bitsrun
  try {
    await init(username, password);
  }
  catch(e) {
    store.status = NetworkStatus.ERROR;
    store.errorMsg = e as string;
    return;
  }

  // 登录
  store.status = NetworkStatus.PENDING;
  const resp = await invoke("login") as ApiResp;
  if (resp.message === "login_error") {
    store.status = NetworkStatus.OFFLINE;
  } else {
    store.status = NetworkStatus.ONLINE;
  }
}

export {
  init,
  login,
  setProxy,
};