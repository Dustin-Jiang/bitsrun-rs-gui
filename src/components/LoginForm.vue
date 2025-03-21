<template>
  <form class="column" @submit.prevent="login">
    <input type="text" v-model="username" :placeholder="i18n.t('username')" />
    <input type="password" v-model="password" :placeholder="i18n.t('password')" />

    <Checkbox v-model="store.userInfo.remember">
      {{ i18n.t('remember_me') }}
    </Checkbox>

    <button type="submit">{{ i18n.t('login') }}</button>
  </form>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import I18n from "../model/i18n";
import { invoke } from "@tauri-apps/api/core";
import store, { NetworkStatus } from "../store";
import { init } from "../model/bitsrun";
import { ApiResp } from "../model/api";

import Checkbox from "../components/Checkbox.vue";

const i18n = I18n.getInstance();

const username = ref(store.userInfo.username);
const password = ref(store.userInfo.password);

async function login() {
  store.userInfo.username = "";
  store.userInfo.password = "";
  if (store.userInfo.remember) {
    store.userInfo.username = username.value;
    store.userInfo.password = password.value;
  }

  // 初始化 bitsrun
  await init(username.value, password.value);

  // 登录
  store.status = NetworkStatus.PENDING;
  const resp = await invoke("login") as ApiResp;
  if (resp.message === "login_error") {
    store.status = NetworkStatus.OFFLINE;
  } else {
    store.status = NetworkStatus.ONLINE;
  }
}

onMounted(() => {
  username.value = store.userInfo.username;
  password.value = store.userInfo.password;
})
</script>
