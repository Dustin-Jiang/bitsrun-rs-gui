<template>
  <Transition>
    <div v-if="props.show" class="disabled"></div>
  </Transition>
</template>

<script setup lang="ts">
const props = defineProps<{
  show: boolean
}>()
</script>

<style scoped>
.disabled {
  position: absolute;
  top: 0px;
  left: 0px;
  width: 100%;
  height: 100%;
  background-color: var(--modal-backdrop-color);
  border-radius: 8px;
  z-index: 100;
}

.v-enter-active,
.v-leave-active {
  transition: opacity 0.167s ease-in-out;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

.v-enter-to,
.v-leave-from {
  opacity: 1;
}
</style>