<template>
  <Modal v-model="showProxySettings">
    <h2>{{ i18n.t("proxy_settings") }}</h2>
    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
      <Checkbox v-model="store.settings.proxy.enable">{{ i18n.t("enable_proxy") }}</Checkbox>
      <Checkbox :disabled="!proxyEnable" v-model="store.settings.proxy.system_proxy">{{ i18n.t("system_proxy") }}
      </Checkbox>

      <Input :disabled="!customProxy" :label="i18n.t('http_proxy')" v-model:value="store.settings.proxy.http_proxy"
        placeholder="http://" :error="httpError" />
      <Input :disabled="!customProxy" :label="i18n.t('https_proxy')" v-model:value="store.settings.proxy.https_proxy"
        placeholder="https://" :error="httpsError" />
    </div>
  </Modal>
  <button class="proxy_settings" @click="() => showProxySettings = true">{{ i18n.t("proxy_settings") }}</button>
</template>

<script setup lang="ts">
import Checkbox from "../components/Checkbox.vue";
import Input from "../components/Input.vue";
import { computed, ref } from "vue";
import I18n from "../model/i18n";
import store from "../store";
import Modal from "./Modal.vue";

const i18n = I18n.getInstance();
const showProxySettings = ref(false);

const proxyEnable = computed(() => store.settings.proxy.enable);
const customProxy = computed(() => !store.settings.proxy.system_proxy);

const httpError = computed(() => store.settings.proxy.http_proxy?.startsWith("http://") === false);
const httpsError = computed(() => store.settings.proxy.https_proxy?.startsWith("https://") === false);
</script>

<style scoped>
.proxy_settings {
  position: fixed;
  bottom: 1rem;
  right: 1rem;
  padding: 0.5rem 1rem;
}
</style>