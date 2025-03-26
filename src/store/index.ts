import { reactive, watch } from "vue";
import type { SrunLoginState } from "../model/api";
import Settings, { defaultSettings } from "./settings";
enum NetworkStatus {
  INIT = "init",
  PENDING = "pending",
  OFFLINE = "offline",
  ONLINE = "online",
  ERROR = "error",
}

interface Store {
  userInfo: {
    username: string;
    password: string;
    remember: boolean;
  };
  keepOnline: boolean;
  initialized: boolean;
  status: NetworkStatus;
  netstat: SrunLoginState;
  settings: Settings;
}

const localStore = JSON.parse(localStorage.getItem("store") ?? "{}") as Store;

const store = reactive<Store>({
  userInfo: {
    username: localStore.userInfo?.username ?? "",
    password: localStore.userInfo?.password ?? "",
    remember: localStore.userInfo?.remember?? false,
  },
  settings: localStore.settings ?? defaultSettings,
  keepOnline: localStore.keepOnline ?? false,

  // 不需要持久化的数据
  initialized: false,
  status: NetworkStatus.INIT,
  netstat: {} as SrunLoginState,
});

watch(store, () => {
  localStorage.setItem("store", JSON.stringify(store));
}, { deep: true })

export default store;
export { NetworkStatus };
