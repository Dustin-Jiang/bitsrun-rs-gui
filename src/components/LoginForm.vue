<template>
  <form class="column" @submit.prevent="handleLogin">
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
import store from "../store";
import { login } from "../model/bitsrun";

import Checkbox from "../components/Checkbox.vue";

const i18n = I18n.getInstance();

const username = ref(store.userInfo.username);
const password = ref(store.userInfo.password);

async function handleLogin() {
  store.userInfo.username = "";
  store.userInfo.password = "";
  if (store.userInfo.remember) {
    store.userInfo.username = username.value;
    store.userInfo.password = password.value;
  }

  await login(username.value, password.value)
}

onMounted(() => {
  username.value = store.userInfo.username;
  password.value = store.userInfo.password;
})
</script>
