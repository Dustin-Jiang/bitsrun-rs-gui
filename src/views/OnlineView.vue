<template>
  <NetStat />
  <div class="container">
    <button @click="logout">{{ i18n.t("logout") }}</button>
  </div>
</template>

<script setup lang="ts">
import NetStat from "../components/NetStat.vue";
import { invoke } from "@tauri-apps/api/core";
import I18n from "../model/i18n";
import store, { NetworkStatus } from "../store";
import { ApiResp } from "../model/api";
import { getNetStat } from "../model/netstat";

const i18n = I18n.getInstance();

async function logout() {
  if (!store.initialized) {
    store.status = NetworkStatus.OFFLINE;
    return;
  }
  const resp = await invoke("logout") as ApiResp;
  if (resp.success === true) {
    store.status = NetworkStatus.OFFLINE;
    return;
  }
  else {
    getNetStat().then((state) => {
      store.netstat = state;
      store.status = NetworkStatus.ONLINE; 
    }).catch((state) => {
      if (state.error === "not_online_error") {
        store.status = NetworkStatus.OFFLINE;
      } else {
        store.status = NetworkStatus.ERROR;
      }
    });
  }
}
</script>

<style scoped>
.container {
  display: flex;
  gap: 0.5rem;
}
</style>

