<template>
  <div class="space-y-1">
    <label v-if="label" class="block text-sm font-medium text-slate-700">
      {{ label }}
    </label>
    <div class="relative">
      <input
        ref="inputRef"
        :value="displayValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :class="[
          'w-full border border-slate-300 text-sm transition-all duration-200',
          'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
          'disabled:bg-slate-100 disabled:text-slate-500 disabled:cursor-not-allowed',
          sizeClasses[size],
          error && 'border-red-500 focus:ring-red-500 focus:border-red-500',
          isRecording && 'ring-2 ring-blue-500 border-blue-500'
        ]"
        @focus="startRecording"
        @blur="stopRecording"
        @keydown="handleKeyDown"
        @keyup="handleKeyUp"
        readonly
      />
      <div v-if="isRecording" class="absolute inset-0 bg-blue-50 border-2 border-blue-500 rounded flex items-center justify-center">
        <span class="text-blue-600 text-sm font-medium">Press keys...</span>
      </div>
      <button
        v-if="modelValue"
        @click="clearHotkey"
        class="absolute right-2 top-1/2 transform -translate-y-1/2 p-1 text-slate-400 hover:text-slate-600 transition-colors"
        type="button"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
    <p v-if="error" class="text-xs text-red-600">{{ error }}</p>
    <p v-else-if="hint" class="text-xs text-slate-500">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

interface Props {
  modelValue?: string | null | undefined;
  label?: string;
  placeholder?: string;
  disabled?: boolean;
  error?: string;
  hint?: string;
  size?: 'sm' | 'md' | 'lg';
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Click to record hotkey',
  disabled: false,
  size: 'md'
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | null | undefined): void;
}>();

const inputRef = ref<HTMLInputElement>();
const isRecording = ref(false);
const pressedKeys = ref<Set<string>>(new Set());

const displayValue = computed(() => {
  if (props.modelValue) {
    return props.modelValue;
  }
  return '';
});

const sizeClasses = {
  sm: 'px-3 py-1.5',
  md: 'px-3 py-2',
  lg: 'px-4 py-2.5'
};

const startRecording = () => {
  if (props.disabled) return;
  isRecording.value = true;
  pressedKeys.value.clear();
};

const stopRecording = () => {
  isRecording.value = false;
  pressedKeys.value.clear();
};

const clearHotkey = () => {
  emit('update:modelValue', null);
};

const handleKeyDown = (event: KeyboardEvent) => {
  event.preventDefault();
  
  const key = event.key.toLowerCase();
  const modifiers = [];
  
  if (event.ctrlKey) modifiers.push('ctrl');
  if (event.altKey) modifiers.push('alt');
  if (event.shiftKey) modifiers.push('shift');
  if (event.metaKey) modifiers.push('meta');
  
  // Ignore modifiers only
  if (modifiers.length === 0 && ['control', 'alt', 'shift', 'meta'].includes(key)) {
    return;
  }
  
  // Add main key
  if (!modifiers.includes(key)) {
    modifiers.push(key);
  }
  
  // Handle special keys
  if (key === 'escape') {
    modifiers.splice(modifiers.indexOf(key), 1);
    modifiers.push('esc');
  } else if (key === ' ') {
    modifiers.splice(modifiers.indexOf(key), 1);
    modifiers.push('space');
  }
  
  const combination = modifiers.join('+');
  emit('update:modelValue', combination);
  
  // Stop recording after getting full combination
  stopRecording();
};

const handleKeyUp = (_: KeyboardEvent) => {
  if (!isRecording.value) return;
  // Can add additional logic when keys are released
};

// Global handlers for capturing keys outside input
const handleGlobalKeyDown = (event: KeyboardEvent) => {
  if (isRecording.value && event.target !== inputRef.value) {
    handleKeyDown(event);
  }
};

const handleGlobalKeyUp = (event: KeyboardEvent) => {
  if (isRecording.value && event.target !== inputRef.value) {
    handleKeyUp(event);
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleGlobalKeyDown);
  document.addEventListener('keyup', handleGlobalKeyUp);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalKeyDown);
  document.removeEventListener('keyup', handleGlobalKeyUp);
});
</script> 