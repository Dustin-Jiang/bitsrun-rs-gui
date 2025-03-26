<script setup lang="ts">
import { ref, provide, computed, onMounted } from "vue";
import { getNetStat } from "./model/netstat";
import I18n, { type Locale } from "./model/i18n";
import store, { NetworkStatus } from "./store";
import OnlineView from "./views/OnlineView.vue";
import { init } from "./model/bitsrun";
import LoginView from "./views/LoginView.vue";
import { ApiResp, SrunLoginState } from "./model/api";
import ErrorView from "./views/ErrorView.vue";

const locale = ref<Locale>("zh_CN");
const i18n = I18n.getInstance();

computed(() => {
  i18n.locale = locale.value;
})
provide("i18n", i18n);

async function initialize() {
  // 获取状态
  getNetStat().then(async (state) => {
    if (store.userInfo.username && store.userInfo.password) {
      await init(store.userInfo.username, store.userInfo.password);
      store.initialized = true;
    }
    store.netstat = state;
    // store.status = NetworkStatus.ONLINE;
    store.status = NetworkStatus.OFFLINE;
  }).catch((status: ApiResp<SrunLoginState>) => {
    store.errorMsg = JSON.stringify(status, null, 2);
    if (status.success == false) {
      store.status = NetworkStatus.ERROR;
    }
    else {
      if (status.message.error === "not_online_error") {
        store.status = NetworkStatus.OFFLINE;
      } else {
        store.status = NetworkStatus.ERROR;
      }
    }
  });
}

onMounted(async () => {
  locale.value = "zh_CN";
  i18n.locale = locale.value;
  await initialize();
});
</script>

<template>
  <main class="container">
    <h1>{{ i18n.t("app_name") }}</h1>
    <LoginView v-if="store.status === NetworkStatus.OFFLINE" />
    <OnlineView v-else-if="store.status === NetworkStatus.ONLINE" />
    <ErrorView v-else />
  </main>
</template>

<style>
body {
  margin: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}

:root {
  font-family: "Inter", Helvetica, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: var(--background-color);

  --background-color: #f6f6f6;
  --primary-color: #396cd8;
  --error-color: #dd0000ca;
  --modal-backdrop-color: #eeeeeec0;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

:root[lang="zh_CN"] {
  font-family: "Inter", Helvetica, "PingFang SC", "Noto Sans SC", "Source Han Sans SC", 
    "Source Han Sans CN","Microsoft YaHei UI", sans-serif;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.column {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  justify-content: center;
}

input[type="text"],
input[type="password"],
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: var(--primary-color);
}
button:active {
  border-color: var(--primary-color);
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    --background-color: #2f2f2f;
    --modal-backdrop-color: #000000b0;
  }

  input[type="text"],
  input[type="password"],
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>

<style scoped>
h1 {
  text-align: center;
  font-size: 1.6rem;
}
</style>