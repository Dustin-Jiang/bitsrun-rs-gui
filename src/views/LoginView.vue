<template>
  <LoginForm />
  <Modal v-model="showProxySettings">
    <ProxySettings />
  </Modal>
  <button class="proxy_settings" @click="() => showProxySettings = true">{{ i18n.t("proxy_settings") }}</button>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import I18n from "../model/i18n";
import store from "../store";

const i18n = I18n.getInstance();
const showProxySettings = ref(false);

import LoginForm from "../components/LoginForm.vue";
import Modal from "../components/Modal.vue";
import ProxySettings from "../components/ProxySettings.vue";
</script>

<style scoped>
.proxy_settings {
  position: fixed;
  bottom: 1rem;
  right: 1rem;
  padding: 0.5rem 1rem;
}
</style>
