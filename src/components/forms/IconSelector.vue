<template>
  <div :class="ui.root()">
    <label :class="ui.label()">{{ label }}</label>
    <div :class="ui.field()">
      <input
        :value="modelValue"
        :placeholder="placeholder"
        :class="ui.input({ class: inputClass })"
        @input="
          $emit('update:modelValue', ($event.target as HTMLInputElement).value)
        "
        @focus="showDropdown = true"
        @blur="handleBlur"
      />
      <button
        type="button"
        :class="ui.toggle()"
        @click="showDropdown = !showDropdown"
      >
        <ChevronDownIcon :class="ui.chevron()" />
      </button>
    </div>

    <div v-if="showDropdown" :class="ui.dropdown()">
      <div :class="ui.dropdownInner()">
        <div :class="ui.grid()">
          <button
            v-for="icon in availableIcons"
            :key="icon"
            type="button"
            :class="
              ui.iconBtn({
                class: modelValue === icon ? ui.iconBtnActive() : undefined,
              })
            "
            @click="selectIcon(icon)"
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
import { tv } from '@/lib/tv';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';

defineOptions({ name: 'IconSelector' });

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

const ui = tv({
  slots: {
    root: 'relative',
    label: 'block text-sm font-medium text-slate-700 mb-1',
    field: 'relative',
    input: [
      'w-full h-8 px-2.5 border border-slate-300 rounded-md text-sm',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    toggle:
      'absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600',
    chevron: 'w-4 h-4',
    dropdown:
      'absolute z-50 mt-1 min-w-[320px] w-[400px] bg-white border border-slate-200 rounded-md shadow-lg max-h-72 overflow-y-auto',
    dropdownInner: 'p-2.5',
    grid: 'grid grid-cols-12 gap-1.5',
    iconBtn:
      'w-8 h-8 flex items-center justify-center text-lg hover:bg-slate-100 rounded-md transition-colors',
    iconBtnActive: 'bg-blue-100 text-blue-600',
  },
})();

// Список доступных иконок
const availableIcons = [
  // Технологии и терминалы
  '⚡',
  '🚀',
  '🖥️',
  '💻',
  '🔧',
  '🐧',
  '🪟',
  '🍏',
  '🐱',
  '📝',
  '📄',
  '⚛️',
  '🔗',
  '🔒',
  '👑',
  '📍',
  '📑',
  '🖱️',
  '⌨️',
  '🖨️',
  '📱',
  '💾',
  '🔍',
  '⚙️',
  '🛠️',
  '🧰',
  '🧲',
  '🧪',
  '🧬',
  '🧫',
  // Состояния и действия
  '✅',
  '❌',
  '🔄',
  '⏳',
  '⌛',
  '🕒',
  '🕹️',
  '🔋',
  '🔌',
  '🔦',
  '🔊',
  '🔉',
  '🔈',
  '🔇',
  '🔔',
  '🔕',
  '💡',
  '🔥',
  '❄️',
  '🌈',
  '⭐',
  '��',
  '✨',
  '💫',
  '🌀',
  '🌪️',
  '🌊',
  '🌋',
  '🌎',
  '🌍',
  '🌏',
  // Разное
  '🎯',
  '🎨',
  '📊',
  '📈',
  '📉',
  '🎲',
  '🎮',
  '🎰',
  '🎵',
  '🎶',
  '🎹',
  '🎸',
  '🎺',
  '🎻',
  '🥁',
  '🏠',
  '🏢',
  '🏭',
  '🏰',
  '🏯',
  '��️',
  '🏟️',
  '🏖️',
  '🏝️',
  '🏔️',
  '🗻',
  '🏕️',
  '🏗️',
  '🏘️',
  '🏚️',
  '🏜️',
  '🏞️',
  '🏡',
  '🏣',
  '🏤',
  '🏥',
  '🏦',
  '🏧',
  '🏨',
  '🏩',
  '🏪',
  '🏫',
  '🏬',
  '🏮',
  '🏯',
  '🏰',
  // Символы и стрелки
  '⬆️',
  '⬇️',
  '⬅️',
  '➡️',
  '↗️',
  '↘️',
  '↙️',
  '↖️',
  '🔼',
  '🔽',
  '⏫',
  '⏬',
  '⏩',
  '⏪',
  '⏭️',
  '⏮️',
  // Эмоции и люди
  '😀',
  '😃',
  '😄',
  '😁',
  '😆',
  '😅',
  '😂',
  '🤣',
  '😊',
  '😇',
  '🙂',
  '🙃',
  '😉',
  '😌',
  '😍',
  '🥰',
  '😘',
  '😗',
  '😙',
  '😚',
  '😋',
  '😜',
  '😝',
  '😛',
  '🤑',
  '🤗',
  '🤩',
  '🤔',
  '🤨',
  '��',
  '😑',
  '😶',
  // Животные
  '🐶',
  '🐱',
  '🐭',
  '🐹',
  '🐰',
  '🦊',
  '🐻',
  '🐼',
  '🐨',
  '🐯',
  '🦁',
  '🐮',
  '🐷',
  '🐸',
  '🐵',
  '🦄',
  // Еда
  '🍏',
  '🍎',
  '🍐',
  '🍊',
  '🍋',
  '🍌',
  '🍉',
  '🍇',
  '🍓',
  '🫐',
  '🍈',
  '🍒',
  '🍑',
  '🥭',
  '🍍',
  '🥥',
  // Транспорт
  '🚗',
  '🚕',
  '🚙',
  '🚌',
  '🚎',
  '🏎️',
  '🚓',
  '🚑',
  '🚒',
  '🚐',
  '🚚',
  '🚛',
  '🚜',
  '🛵',
  '🏍️',
  '🚲',
  // Флаги (несколько)
  '🇷🇺',
  '🇺🇸',
  '🇬🇧',
  '🇩🇪',
  '🇫🇷',
  '🇨🇳',
  '🇯🇵',
  '🇰🇷',
  '🇮🇳',
  '🇧🇷',
  '🇦🇺',
  '🇨🇦',
  '🇪🇸',
  '🇮🇹',
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
