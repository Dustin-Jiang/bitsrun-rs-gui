<template>
  <div>
    <label>{{ props.label }}</label>
    <div style="position: relative;">
      <Disable :show="props.disabled" />
      <input :type="props.type ?? 'text'" v-model="value" :placeholder="props.placeholder ?? ''" :class="error ?? false ? 'error' : ''" />
    </div>
  </div>
</template>

<script setup lang="ts">
import Disable from './Disable.vue';

type InputTypeHTMLAttribute = 'button' | 'checkbox' | 'color' | 'date' | 'datetime-local' | 'email' | 'file' | 'hidden' | 'image' | 'month' | 'number' | 'password' | 'radio' | 'range' | 'reset' | 'search' | 'submit' | 'tel' | 'text' | 'time' | 'url' | 'week' | (string & {});

const props = defineProps<{
  label?: string,
  type?: InputTypeHTMLAttribute,
  placeholder?: string
  disabled?: boolean
  error?: boolean
}>()

const value = defineModel("value", { type: String })
</script>

<style scoped>
label {
  display: block;
  margin-bottom: 0.4rem;
  font-size: 1em;
  font-weight: 500;
  margin-inline-start: 0.4rem;
}

input.error {
  border-color: var(--error-color) !important;
}

input:active,
input:focus {
  border-color: var(--primary-color);
}
</style>