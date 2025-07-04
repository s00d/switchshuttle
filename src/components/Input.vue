<template>
  <div class="space-y-1">
    <label v-if="label" :for="id" class="block text-sm font-medium text-slate-700">
      {{ label }}
    </label>
    <div class="relative">
      <select
        v-if="type === 'select'"
        :id="id"
        :value="modelValue"
        :disabled="disabled"
        :class="[
          'w-full border border-slate-300 text-sm transition-all duration-200',
          'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
          'disabled:bg-slate-100 disabled:text-slate-500 disabled:cursor-not-allowed',
          sizeClasses[size],
          error && 'border-red-500 focus:ring-red-500 focus:border-red-500',
          inputClass
        ]"
        @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
        @blur="$emit('blur')"
        @focus="$emit('focus')"
      >
        <option v-for="option in options" :key="option.value" :value="option.value">
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
        :class="[
          'w-full border border-slate-300 text-sm transition-all duration-200',
          'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
          'disabled:bg-slate-100 disabled:text-slate-500 disabled:cursor-not-allowed',
          sizeClasses[size],
          error && 'border-red-500 focus:ring-red-500 focus:border-red-500',
          inputClass
        ]"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        @blur="$emit('blur')"
        @focus="$emit('focus')"
        @keydown="$emit('keydown', $event)"
      />
      <slot name="suffix" />
    </div>
    <p v-if="error" class="text-xs text-red-600">{{ error }}</p>
    <p v-else-if="hint" class="text-xs text-slate-500">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
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
}

withDefaults(defineProps<Props>(), {
  modelValue: '',
  type: 'text',
  disabled: false,
  size: 'md',
  options: () => [],
  inputClass: ''
});

defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'blur'): void;
  (e: 'focus'): void;
  (e: 'keydown', event: KeyboardEvent): void;
}>();

const sizeClasses = {
  sm: 'px-3 py-1.5',
  md: 'px-3 py-2',
  lg: 'px-4 py-2.5'
};
</script> 