<template>
  <div class="container">
    <span>
      <slot />
    </span>
    <label for="toggle" class="root">
      <input type="checkbox" id="toggle" v-model="model">
      <span></span>
    </label>
  </div>
</template>

<script lang="ts" setup>
const model = defineModel({ type: Boolean })
</script>

<style scoped>
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
}
</style>