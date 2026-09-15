<template>
  <div :class="ui.root()">
    <label v-if="label" :for="id" :class="ui.label()">
      {{ label }}
    </label>
    <div :class="ui.controlWrap()">
      <select
        v-if="type === 'select'"
        :id="id"
        :value="modelValue"
        :disabled="disabled"
        :class="ui.control({ class: inputClass })"
        @change="
          $emit('update:modelValue', ($event.target as HTMLSelectElement).value)
        "
        @blur="$emit('blur')"
        @focus="$emit('focus')"
      >
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
        >
          {{ option.label }}
        </option>
      </select>

      <input
        v-else
        :id="id"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :min="min"
        :max="max"
        :class="ui.control({ class: inputClass })"
        @input="
          $emit('update:modelValue', ($event.target as HTMLInputElement).value)
        "
        @blur="$emit('blur')"
        @focus="$emit('focus')"
        @keydown="$emit('keydown', $event)"
      />
      <slot name="suffix" />
    </div>
    <p v-if="error" :class="ui.error()">{{ error }}</p>
    <p v-else-if="hint" :class="ui.hint()">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { inputTv } from './themes';

defineOptions({ name: 'Input' });

interface Option {
  value: string;
  label: string;
}

interface Props {
  modelValue?: string;
  label?: string;
  placeholder?: string;
  type?: string;
  id?: string;
  disabled?: boolean;
  error?: string;
  hint?: string;
  size?: 'sm' | 'md' | 'lg';
  options?: Option[];
  inputClass?: string;
  min?: number;
  max?: number;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  type: 'text',
  disabled: false,
  size: 'md',
  options: () => [],
  inputClass: '',
});

defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'blur'): void;
  (e: 'focus'): void;
  (e: 'keydown', event: KeyboardEvent): void;
}>();

const ui = computed(() =>
  inputTv({
    size: props.size,
    isError: !!props.error,
  }),
);
</script>
