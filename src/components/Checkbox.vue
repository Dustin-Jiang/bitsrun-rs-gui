<template>
  <div class="container">
    <Disable :show="props.disabled" />
    <span>
      <slot />
    </span>
    <label :for="id" class="root" :disabled="props.disabled">
      <input type="checkbox" :id="id" v-model="model" :disabled="props.disabled">
      <span></span>
    </label>
  </div>
</template>

<script lang="ts" setup>
import Disable from './Disable.vue';

const model = defineModel({ type: Boolean })
const props = defineProps({
  disabled: {
    type: Boolean,
    default: false
  }
})
const id = `toggle-${Math.random().toString(36).substr(2, 9)}`
</script>

<style scoped>
.container {
  position: relative;
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em !important;
  box-sizing: border-box;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

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

.root {
  --button-width: 50px;
  --button-height: 29.5px;
  --toggle-diameter: 25.5px;
  --button-toggle-offset: calc((var(--button-height) - var(--toggle-diameter)) / 2);
  --toggle-shadow-offset: 1.0px;
  --toggle-wider: 33.3px;
  --color-grey: #E9E9E9;
  --color-dark-grey: #39393D;
  --color-green: var(--primary-color);

  display: flex;
  justify-content: center;
  align-items: center;
}

.container {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  gap: space-between;
  width: 100%;
}

span {
  display: inline-block;
  width: var(--button-width);
  height: var(--button-height);
  background-color: var(--color-grey);
  border-radius: calc(var(--button-height) / 2);
  position: relative;
  transition: .3s all ease-in-out;
}

span::after {
  content: '';
  display: inline-block;
  width: var(--toggle-diameter);
  height: var(--toggle-diameter);
  background-color: #fff;
  border-radius: calc(var(--toggle-diameter) / 2);
  position: absolute;
  top: var(--button-toggle-offset);
  transform: translateX(var(--button-toggle-offset));
  box-shadow: var(--toggle-shadow-offset) 0 calc(var(--toggle-shadow-offset) * 4) rgba(0, 0, 0, .10);
  transition: .3s all ease-in-out;
}

input[type="checkbox"]:checked + span {
  background-color: var(--color-green);
}

input[type="checkbox"]:checked + span::after {
  transform: translateX(calc(var(--button-width) - var(--toggle-diameter) - var(--button-toggle-offset)));
  box-shadow: calc(var(--toggle-shadow-offset) * -1) 0 calc(var(--toggle-shadow-offset) * 4) rgba(0, 0, 0, .10);
}

input[type="checkbox"] {
  display: none;
}

input[type="checkbox"]:active + span::after {
  width: var(--toggle-wider);
}

input[type="checkbox"]:checked:active + span::after {
  transform: translateX(calc(var(--button-width) - var(--toggle-wider) - var(--button-toggle-offset)));
}

@media(prefers-color-scheme: dark) {
  span {
    background-color: var(--color-dark-grey);
  }
  .container {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
}
</style>