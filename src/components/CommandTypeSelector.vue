<template>
  <div class="space-y-4 p-4 bg-slate-50 rounded-lg border border-slate-100">
    <label class="block text-sm font-semibold text-slate-700">Command Type</label>
    <div class="flex space-x-2">
      <button
        @click="selectType('single')"
        :class="[
          'flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
          modelValue === 'single'
            ? 'bg-blue-600 text-white shadow-md'
            : 'bg-white text-slate-700 border border-slate-200 hover:bg-slate-50 hover:border-slate-300'
        ]"
      >
        Single Command
      </button>
      <button
        @click="selectType('multiple')"
        :class="[
          'flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
          modelValue === 'multiple'
            ? 'bg-blue-600 text-white shadow-md'
            : 'bg-white text-slate-700 border border-slate-200 hover:bg-slate-50 hover:border-slate-300'
        ]"
      >
        Multiple Commands
      </button>
      <button
        @click="selectType('submenu')"
        :class="[
          'flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200',
          modelValue === 'submenu'
            ? 'bg-blue-600 text-white shadow-md'
            : 'bg-white text-slate-700 border border-slate-200 hover:bg-slate-50 hover:border-slate-300'
        ]"
      >
        Submenu
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const selectType = (type: string) => {
  emit('update:modelValue', type);
};
</script> 