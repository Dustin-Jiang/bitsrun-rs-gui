<template>
  <Transition>
    <div v-if="model" class="modal-backdrop" @click.stop="() => model = false">
      <div class="modal-content" @click.stop="() => {}">
        <slot></slot>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
const model = defineModel({ type: Boolean })
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;

  display: flex;
  justify-content: center;
  align-items: center;

  background-color: var(--modal-backdrop-color);
  z-index: 1000;
}

.modal-content {
  padding: 1rem;
  border-radius: 8px;
  background-color: var(--background-color);
  z-index: 1001;
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