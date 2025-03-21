<template>
  <NetStat />
  <div class="container" :title="keepOnlineInfo">
    <Checkbox v-model="store.keepOnline" :disabled="keepOnlineDisabled">
      {{ i18n.t("keep_online") }}
    </Checkbox>
    <button @click="logout">{{ i18n.t("logout") }}</button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from "vue";
import NetStat from "../components/NetStat.vue";
import { invoke } from "@tauri-apps/api/core";
import I18n from "../model/i18n";
import store, { NetworkStatus } from "../store";
import { ApiResp } from "../model/api";
import { getNetStat } from "../model/netstat";

import Checkbox from "../components/Checkbox.vue";
import { login } from "../model/bitsrun";

const i18n = I18n.getInstance();

const keepOnlineInfo = computed(() => {
  return i18n.t("keep_online_info");
})

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

const keepOnlineDisabled = computed(() => {
  const result = !store.userInfo.password || !store.userInfo.username;
  if (result) {
    store.keepOnline = false;
  }
  return result;
})

const INTERVAL = 60;
onMounted(() => {
  let delta = INTERVAL;
  setInterval(() => {
    if (!store.keepOnline) {
      delta = 0;
      return ;
    }
    if (delta == 0) {
      delta = INTERVAL;
      getNetStat().then((state) => {
        store.netstat = state;
        store.status = NetworkStatus.ONLINE; 
      }).catch(async (state) => {
        if (state.error === "not_online_error") {
          await login(store.userInfo.username, store.userInfo.password)
        } else {
          store.status = NetworkStatus.ERROR;
        }
      });
    }
    if (store.keepOnline) {
      delta -= 1;
      return;
    }
  }, 1000)
})
</script>

<style scoped>
.container {
  display: flex;
  gap: 0.5rem;
  padding: 0.5rem 0;
}
</style>

