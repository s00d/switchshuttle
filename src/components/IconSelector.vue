<template>
  <div class="relative">
    <label class="block text-sm font-medium text-slate-700 mb-2">{{ label }}</label>
    <div class="relative">
      <input
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        :placeholder="placeholder"
        class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        :class="inputClass"
        @focus="showDropdown = true"
        @blur="handleBlur"
      />
      <button
        type="button"
        @click="showDropdown = !showDropdown"
        class="absolute right-2 top-1/2 transform -translate-y-1/2 text-slate-400 hover:text-slate-600"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
    </div>
    
    <!-- Dropdown -->
    <div
      v-if="showDropdown"
      class="absolute z-50 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg max-h-72 overflow-y-auto"
      style="min-width: 320px; width: 400px;"
    >
      <div class="p-2">
        <div class="grid grid-cols-12 gap-2">
          <button
            v-for="icon in availableIcons"
            :key="icon"
            @click="selectIcon(icon)"
            class="w-8 h-8 flex items-center justify-center text-lg hover:bg-slate-100 rounded transition-colors"
            :class="{ 'bg-blue-100 text-blue-600': modelValue === icon }"
          >
            {{ icon }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

defineProps<{
  modelValue: string;
  label?: string;
  placeholder?: string;
  inputClass?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const showDropdown = ref(false);

// Список доступных иконок
const availableIcons = [
  // Технологии и терминалы
  '⚡', '🚀', '🖥️', '💻', '🔧', '🐧', '🪟', '🍏', '🐱', '📝', '📄', '⚛️', '🔗', '🔒', '👑',
  '📍', '📑', '🖱️', '⌨️', '🖨️', '📱', '💾', '🔍', '⚙️', '🛠️', '🧰', '🧲', '🧪', '🧬', '🧫',
  // Состояния и действия
  '✅', '❌', '🔄', '⏳', '⌛', '🕒', '🕹️', '🔋', '🔌', '🔦', '🔊', '🔉', '🔈', '🔇', '🔔', '🔕',
  '💡', '🔥', '❄️', '🌈', '⭐', '��', '✨', '💫', '🌀', '🌪️', '🌊', '🌋', '🌎', '🌍', '🌏',
  // Разное
  '🎯', '🎨', '📊', '📈', '📉', '🎲', '🎮', '🎰', '🎵', '🎶', '🎹', '🎸', '🎺', '🎻', '🥁',
  '🏠', '🏢', '🏭', '🏰', '🏯', '��️', '🏟️', '🏖️', '🏝️', '🏔️', '🗻', '🏕️', '🏗️', '🏘️', '🏚️',
  '🏜️', '🏞️', '🏡', '🏣', '🏤', '🏥', '🏦', '🏧', '🏨', '🏩', '🏪', '🏫', '🏬', '🏮', '🏯', '🏰',
  // Символы и стрелки
  '⬆️', '⬇️', '⬅️', '➡️', '↗️', '↘️', '↙️', '↖️', '🔼', '🔽', '⏫', '⏬', '⏩', '⏪', '⏭️', '⏮️',
  // Эмоции и люди
  '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣', '😊', '😇', '🙂', '🙃', '😉', '😌', '😍', '🥰',
  '😘', '😗', '😙', '😚', '😋', '😜', '😝', '😛', '🤑', '🤗', '🤩', '🤔', '🤨', '��', '😑', '😶',
  // Животные
  '🐶', '🐱', '🐭', '🐹', '🐰', '🦊', '🐻', '🐼', '🐨', '🐯', '🦁', '🐮', '🐷', '🐸', '🐵', '🦄',
  // Еда
  '🍏', '🍎', '🍐', '🍊', '🍋', '🍌', '🍉', '🍇', '🍓', '🫐', '🍈', '🍒', '🍑', '🥭', '🍍', '🥥',
  // Транспорт
  '🚗', '🚕', '🚙', '🚌', '🚎', '🏎️', '🚓', '🚑', '🚒', '🚐', '🚚', '🚛', '🚜', '🛵', '🏍️', '🚲',
  // Флаги (несколько)
  '🇷🇺', '🇺🇸', '🇬🇧', '🇩🇪', '🇫🇷', '🇨🇳', '🇯🇵', '🇰🇷', '🇮🇳', '🇧🇷', '🇦🇺', '🇨🇦', '🇪🇸', '🇮🇹',
];

const selectIcon = (icon: string) => {
  emit('update:modelValue', icon);
  showDropdown.value = false;
};

const handleBlur = () => {
  // Небольшая задержка, чтобы успеть кликнуть на иконку
  setTimeout(() => {
    showDropdown.value = false;
  }, 150);
};
</script> 