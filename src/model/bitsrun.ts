import { invoke } from "@tauri-apps/api/core";
import store, { NetworkStatus } from "../store";

const init = async (username: string, password: string) => {
  await invoke("init_bitsrun", { config: {
    username: username,
    password: password
  } });
  store.status = NetworkStatus.INIT;
}

export { init };