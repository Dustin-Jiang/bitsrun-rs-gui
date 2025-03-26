<template>
  <div style="display: flex; flex-direction: column; gap: 1rem;">
    <h2>{{ i18n.t("error") }}</h2>
    <pre>
      <code>{{ store.errorMsg }}</code>
    </pre>

    <button @click="() => retry()">{{ i18n.t("retry") }}</button>
  </div>
  <ProxySettings />
</template>

<script setup lang="ts">
import ProxySettings from '../components/ProxySettings.vue';
import I18n from '../model/i18n';
import store, { NetworkStatus } from '../store';

const i18n = I18n.getInstance();

const retry = () => {
  store.status = NetworkStatus.OFFLINE;
}
</script>

<style scoped>
pre, code {
  font-family: "Consolas", monospace;
}

pre {
  overflow: auto;
  max-height: 50vh;
  max-width: 80vw;
}
</style>