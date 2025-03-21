<template>
  <table>
    <tr>
      <td>用户名</td>
      <td>{{ username }}</td>
    </tr>
    <tr>
      <td>IP地址</td>
      <td>{{ ip }}</td>
    </tr>
    <tr>
      <td>登录时间</td>
      <td>{{ loginTimestamp.toLocaleString() }}</td>
    </tr>
    <tr>
      <td>在线时间</td>
      <td>{{ toHumanReadable(onlineTime) }}</td>
    </tr>
    <tr>
      <td>已用流量</td>
      <td>{{ usedFlow }} GB</td>
    </tr>
  </table>
</template>

<script setup lang="ts">
import store from "../store";
import { computed, ref, onMounted } from "vue";
import I18n from "../model/i18n";
import { getNetStat } from "../model/netstat";

const i18n = I18n.getInstance();

const username = computed(() => store.netstat.user_name);
const ip = computed(() => store.netstat.online_ip);

const loginTimestamp = computed(() => {
  if (store.netstat.add_time) {
    return new Date(store.netstat.add_time * 1000);
  }
  return new Date();
});

const toHumanReadable = (timestamp: number) => {
  const days = Math.max(0, Math.floor(timestamp / 86400));
  const hours = Math.max(0, Math.floor((timestamp % 86400) / 3600));
  const minutes = Math.max(0, Math.floor((timestamp % 3600) / 60));
  const seconds = Math.max(0, timestamp % 60);
  return i18n.format("online_time", days.toString(), hours.toString(), minutes.toString(), seconds.toString());
};

const now = ref(new Date().getTime());
const onlineTime = computed(() => {
  const delta = now.value - loginTimestamp.value.getTime()
  return Math.floor(delta / 1000);
});

onMounted(async () => {
  setInterval(() => {
    now.value = new Date().getTime();
  }, 1000);

  if (!username.value || !ip.value) {
    store.netstat = await getNetStat()
  }
});

const usedFlow = computed(() => {
  return ((store.netstat.sum_bytes ?? 0) / 1024 / 1024 / 1024).toFixed(2);
});
</script>

<style scoped>
td {
  padding: 0.5rem;
}

tr > td:first-child {
  /* text-align: right; */
  font-weight: bold;
}

tr > td {
  text-align: left;
}
</style>
