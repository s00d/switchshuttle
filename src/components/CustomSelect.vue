<template>
  <div class="relative">
    <button
      @click="toggleDropdown"
      type="button"
      class="w-full px-3 py-2 h-10 text-left bg-white border border-slate-300 text-sm transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
      :class="{ 'ring-2 ring-blue-500 border-blue-500': isOpen }"
    >
      <div class="flex items-center justify-between h-full">
        <div class="flex items-center space-x-2">
          <span v-if="selectedOption?.icon" class="text-lg">{{ selectedOption.icon }}</span>
          <span class="text-slate-900">{{ selectedOption?.label || placeholder }}</span>
        </div>
        <svg 
          class="w-5 h-5 text-slate-400 transition-transform duration-200"
          :class="{ 'rotate-180': isOpen }"
          fill="none" 
          stroke="currentColor" 
          viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </div>
    </button>

    <!-- Dropdown -->
    <div 
      v-if="isOpen"
      class="absolute z-50 w-full mt-1 bg-white border border-slate-300 rounded-lg shadow-lg max-h-60 overflow-auto"
    >
      <div class="py-1">
        <div
          v-for="option in options"
          :key="option.value"
          @click="selectOption(option)"
          class="flex items-center space-x-3 px-3 py-2 cursor-pointer hover:bg-slate-50 transition-all duration-200 text-sm"
          :class="{ 'bg-blue-50 text-blue-700': option.value === modelValue }"
        >
          <span v-if="option.icon" class="text-lg">{{ option.icon }}</span>
          <span class="flex-1">{{ option.label }}</span>
          <svg 
            v-if="option.value === modelValue"
            class="w-4 h-4 text-blue-600" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

interface Option {
  value: string;
  label: string;
  icon?: string;
}

interface Props {
  modelValue: string;
  options: Option[];
  placeholder?: string;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Select an option'
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const isOpen = ref(false);

const selectedOption = computed(() => {
  return props.options.find(option => option.value === props.modelValue);
});

function toggleDropdown() {
  isOpen.value = !isOpen.value;
}

function selectOption(option: Option) {
  emit('update:modelValue', option.value);
  isOpen.value = false;
}

function handleClickOutside(event: Event) {
  const target = event.target as HTMLElement;
  if (!target.closest('.relative')) {
    isOpen.value = false;
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script> 