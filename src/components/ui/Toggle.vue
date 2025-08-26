<template>
  <div class="flex items-start justify-between gap-4">
    <div class="flex flex-col flex-1 min-w-0">
      <label :for="id" class="text-sm font-medium text-slate-700 break-words">{{
        label
      }}</label>
      <p v-if="description" class="text-xs text-slate-500 mt-1 break-words">
        {{ description }}
      </p>
    </div>
    <button
      :id="id"
      type="button"
      :class="[
        'relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 flex-shrink-0',
        modelValue ? 'bg-blue-600' : 'bg-slate-200',
      ]"
      :disabled="disabled"
      @click="$emit('update:modelValue', !modelValue)"
    >
      <span
        :class="[
          'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
          modelValue ? 'translate-x-6' : 'translate-x-1',
        ]"
      />
    </button>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean;
  label: string;
  description?: string;
  id?: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  disabled: false,
});

defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();
</script>
