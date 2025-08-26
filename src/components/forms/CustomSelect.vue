<template>
  <div class="relative">
    <button
        type="button"
        class="w-full px-3 py-2 h-10 text-left bg-white border border-slate-300 text-sm transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        :class="{ 'ring-2 ring-blue-500 border-blue-500': isOpen }"
        @click="toggleDropdown"
    >
      <div class="flex items-center justify-between h-full">
        <div class="flex items-center space-x-2 min-w-0 flex-1">
          <span v-if="selectedOption?.icon" class="text-lg flex-shrink-0">{{
              selectedOption.icon
            }}</span>
          <div class="min-w-0 flex-1">
            <div class="text-slate-900 truncate">
              {{ selectedOption?.label || placeholder }}
            </div>
            <div
                v-if="selectedOption?.description"
                class="text-xs text-slate-500 truncate"
            >
              {{ selectedOption.description }}
            </div>
          </div>
        </div>
        <ChevronDownIcon
            class="w-5 h-5 text-slate-400 transition-transform duration-200 flex-shrink-0 ml-2"
            :class="{ 'rotate-180': isOpen }"
        />
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
            class="flex items-start space-x-3 px-3 py-2 cursor-pointer hover:bg-slate-50 transition-all duration-200 text-sm"
            :class="{ 'bg-blue-50 text-blue-700': option.value === modelValue }"
            @click="selectOption(option)"
        >
          <span v-if="option.icon" class="text-lg flex-shrink-0 mt-0.5">{{
              option.icon
            }}</span>
          <div class="min-w-0 flex-1">
            <div class="font-medium">{{ option.label }}</div>
            <div
                v-if="option.description"
                class="text-xs text-slate-500 mt-0.5"
            >
              {{ option.description }}
            </div>
          </div>
          <CheckIcon
              v-if="option.value === modelValue"
              class="w-4 h-4 text-blue-600 flex-shrink-0 mt-0.5"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import CheckIcon from '../icons/CheckIcon.vue';

interface Option {
  value: string;
  label: string;
  icon?: string;
  description?: string;
}

interface Props {
  modelValue: string;
  options: Option[];
  placeholder?: string;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Select an option',
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
