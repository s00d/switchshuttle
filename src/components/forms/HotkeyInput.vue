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
          isRecording && 'ring-2 ring-blue-500 border-blue-500',
        ]"
        readonly
        @focus="startRecording"
        @blur="stopRecording"
        @keydown="handleKeyDown"
        @keyup="handleKeyUp"
      />
      <div
        v-if="isRecording"
        class="absolute inset-0 bg-blue-50 border-2 border-blue-500 rounded flex items-center justify-center"
      >
        <span class="text-blue-600 text-sm font-medium">Press keys...</span>
      </div>
      <CustomButton
        v-if="modelValue"
        variant="ghost"
        class="absolute right-2 top-1/2 transform -translate-y-1/2 p-1"
        @click="clearHotkey"
      >
        <XIcon class="w-4 h-4" />
      </CustomButton>

      <!-- Dropdown for available keys -->
      <CustomButton
        variant="ghost"
        class="absolute right-8 top-1/2 transform -translate-y-1/2 p-1"
        title="Select from available keys"
        @click="toggleDropdown"
      >
        <ChevronDownIcon class="w-4 h-4" />
      </CustomButton>

      <!-- Teleported Dropdown -->
      <Teleport to="body">
        <div
          v-if="showDropdown"
          class="hotkey-dropdown bg-white border border-slate-200 rounded-lg shadow-lg"
          :style="dropdownStyle"
        >
          <div class="p-2 flex-1 overflow-y-auto" style="max-height: 350px">
            <div class="space-y-2">
              <!-- Modifiers -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Modifiers
                </h4>
                <div class="grid grid-cols-2 gap-1">
                  <CustomButton
                    v-for="modifier in modifiers"
                    :key="modifier.value"
                    variant="ghost"
                    class="px-2 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedModifiers.includes(modifier.value),
                    }"
                    @click="selectModifier(modifier.value)"
                  >
                    {{ modifier.label }}
                  </CustomButton>
                </div>
              </div>

              <!-- Letters -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Letters
                </h4>
                <div class="grid grid-cols-8 gap-1">
                  <CustomButton
                    v-for="letter in letters"
                    :key="letter"
                    variant="ghost"
                    class="px-2 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedKey === letter,
                    }"
                    @click="selectKey(letter)"
                  >
                    {{ letter.toUpperCase() }}
                  </CustomButton>
                </div>
              </div>

              <!-- Numbers -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Numbers
                </h4>
                <div class="grid grid-cols-10 gap-1">
                  <CustomButton
                    v-for="num in numbers"
                    :key="num"
                    variant="ghost"
                    class="px-2 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedKey === num,
                    }"
                    @click="selectKey(num)"
                  >
                    {{ num }}
                  </CustomButton>
                </div>
              </div>

              <!-- Function Keys -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Function Keys
                </h4>
                <div class="grid grid-cols-6 gap-1">
                  <CustomButton
                    v-for="fkey in functionKeys"
                    :key="fkey"
                    variant="ghost"
                    class="px-2 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedKey === fkey,
                    }"
                    @click="selectKey(fkey)"
                  >
                    {{ fkey.toUpperCase() }}
                  </CustomButton>
                </div>
              </div>

              <!-- Special Keys -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Special Keys
                </h4>
                <div class="grid grid-cols-3 gap-1">
                  <CustomButton
                    v-for="special in specialKeys"
                    :key="special.value"
                    variant="ghost"
                    class="px-1 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedKey === special.value,
                    }"
                    @click="selectKey(special.value)"
                  >
                    {{ special.label }}
                  </CustomButton>
                </div>
              </div>

              <!-- Navigation Keys -->
              <div>
                <h4
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide mb-2"
                >
                  Navigation
                </h4>
                <div class="grid grid-cols-4 gap-1">
                  <CustomButton
                    v-for="nav in navigationKeys"
                    :key="nav.value"
                    variant="ghost"
                    class="px-2 py-1 text-xs border border-slate-200 rounded"
                    :class="{
                      'bg-blue-100 text-blue-700 border-blue-300':
                        selectedKey === nav.value,
                    }"
                    @click="selectKey(nav.value)"
                  >
                    {{ nav.label }}
                  </CustomButton>
                </div>
              </div>
            </div>
          </div>

          <!-- Apply Button - Fixed at bottom -->
          <div class="p-2 border-t border-slate-200 bg-white rounded-b-lg">
            <CustomButton
              class="w-full text-sm py-2"
              variant="primary"
              :disabled="selectedModifiers.length === 0 && !selectedKey"
              @click="applyCombination"
            >
              Apply Combination
            </CustomButton>
          </div>
        </div>
      </Teleport>
    </div>

    <p v-if="error" class="text-xs text-red-600">{{ error }}</p>
    <p v-else-if="hint" class="text-xs text-slate-500">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import XIcon from '../icons/XIcon.vue';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

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
  size: 'md',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | null | undefined): void;
}>();

const inputRef = ref<HTMLInputElement>();
const isRecording = ref(false);
const pressedKeys = ref<Set<string>>(new Set());
const showDropdown = ref(false);
const selectedModifiers = ref<string[]>([]);
const selectedKey = ref<string>('');
const dropdownPosition = ref({ top: 0, left: 0, width: 300 });
const isDropdownOpen = ref(false);

const displayValue = computed(() => {
  if (props.modelValue) {
    return props.modelValue;
  }
  return '';
});

const dropdownStyle = computed(() => ({
  position: 'fixed' as const,
  top: `${dropdownPosition.value.top}px`,
  left: `${dropdownPosition.value.left}px`,
  width: `${dropdownPosition.value.width}px`,
  zIndex: 50,
}));

const sizeClasses = {
  sm: 'px-3 py-1.5',
  md: 'px-3 py-2',
  lg: 'px-4 py-2.5',
};

// Available keys based on hotkeys.rs
const modifiers = [
  { value: 'Ctrl', label: 'Ctrl' },
  { value: 'Alt', label: 'Alt' },
  { value: 'Shift', label: 'Shift' },
  { value: 'Meta', label: 'Meta' },
];

const letters = [
  'a',
  'b',
  'c',
  'd',
  'e',
  'f',
  'g',
  'h',
  'i',
  'j',
  'k',
  'l',
  'm',
  'n',
  'o',
  'p',
  'q',
  'r',
  's',
  't',
  'u',
  'v',
  'w',
  'x',
  'y',
  'z',
];

const numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const functionKeys = [
  'f1',
  'f2',
  'f3',
  'f4',
  'f5',
  'f6',
  'f7',
  'f8',
  'f9',
  'f10',
  'f11',
  'f12',
];

const specialKeys = [
  { value: 'space', label: 'Space' },
  { value: 'enter', label: 'Enter' },
  { value: 'tab', label: 'Tab' },
  { value: 'escape', label: 'Escape' },
  { value: 'backspace', label: 'Backspace' },
  { value: 'delete', label: 'Delete' },
  { value: 'insert', label: 'Insert' },
  { value: 'home', label: 'Home' },
  { value: 'end', label: 'End' },
  { value: 'pageup', label: 'Page Up' },
  { value: 'pagedown', label: 'Page Down' },
];

const navigationKeys = [
  { value: 'up', label: '↑' },
  { value: 'down', label: '↓' },
  { value: 'left', label: '←' },
  { value: 'right', label: '→' },
];

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

const updateDropdownPosition = () => {
  if (inputRef.value && showDropdown.value) {
    const rect = inputRef.value.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const dropdownHeight = 400; // Approximate height

    let top = rect.bottom + 4; // 4px gap

    // Check if dropdown would go below viewport
    if (top + dropdownHeight > viewportHeight) {
      top = rect.top - dropdownHeight - 4; // Show above input
    }

    dropdownPosition.value = {
      top,
      left: rect.left,
      width: Math.max(300, rect.width),
    };
  }
};

const toggleDropdown = async () => {
  showDropdown.value = !showDropdown.value;
  if (showDropdown.value) {
    isDropdownOpen.value = true;
    // Calculate position
    await nextTick();
    updateDropdownPosition();

    // Parse current hotkey if exists
    if (props.modelValue) {
      const parts = props.modelValue.split('+');
      selectedModifiers.value = parts.filter(part =>
        ['Ctrl', 'Alt', 'Shift', 'Meta'].includes(part)
      );
      const mainKey =
        parts.find(part => !['Ctrl', 'Alt', 'Shift', 'Meta'].includes(part)) ||
        '';
      // Convert to lowercase for matching with our arrays
      selectedKey.value = mainKey.toLowerCase();
    } else {
      selectedModifiers.value = [];
      selectedKey.value = '';
    }
  } else {
    isDropdownOpen.value = false;
  }
};

const selectModifier = (modifier: string) => {
  const index = selectedModifiers.value.indexOf(modifier);
  if (index > -1) {
    selectedModifiers.value.splice(index, 1);
  } else {
    selectedModifiers.value.push(modifier);
  }
};

const selectKey = (key: string) => {
  selectedKey.value = key;
};

const applyCombination = () => {
  if (selectedModifiers.value.length > 0 || selectedKey.value) {
    const combination = [...selectedModifiers.value, selectedKey.value]
      .filter(Boolean)
      .join('+');
    const formattedCombination = formatHotkey(combination);
    emit('update:modelValue', formattedCombination);
    showDropdown.value = false;
    isDropdownOpen.value = false;
  }
};

const formatHotkey = (hotkey: string): string => {
  return hotkey
    .split('+')
    .map(part => {
      const trimmed = part.trim();
      switch (trimmed.toLowerCase()) {
        case 'ctrl':
          return 'Ctrl';
        case 'alt':
          return 'Alt';
        case 'shift':
          return 'Shift';
        case 'meta':
          return 'Meta';
        default:
          // For letters - uppercase, for the rest - as is
          if (trimmed.length === 1 && /[a-z]/.test(trimmed)) {
            return trimmed.toUpperCase();
          }
          return trimmed;
      }
    })
    .join('+');
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
  if (
    modifiers.length === 0 &&
    ['control', 'alt', 'shift', 'meta'].includes(key)
  ) {
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

const handleKeyUp = () => {
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
    handleKeyUp();
  }
};

// Close dropdown when clicking outside
const handleClickOutside = (event: Event) => {
  if (showDropdown.value) {
    const target = event.target as Element;
    const input = inputRef.value;

    if (!target.closest('.hotkey-dropdown') && target !== input) {
      showDropdown.value = false;
      isDropdownOpen.value = false;
    }
  }
};

// Handle escape key
const handleEscape = (event: KeyboardEvent) => {
  if (showDropdown.value && event.key === 'Escape') {
    showDropdown.value = false;
    isDropdownOpen.value = false;
  }
};

// Handle scroll and resize
const handleScroll = () => {
  if (isDropdownOpen.value) {
    updateDropdownPosition();
  }
};

const handleResize = () => {
  if (isDropdownOpen.value) {
    updateDropdownPosition();
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleGlobalKeyDown);
  document.addEventListener('keyup', handleGlobalKeyUp);
  document.addEventListener('click', handleClickOutside);
  document.addEventListener('keydown', handleEscape);
  window.addEventListener('scroll', handleScroll, true);
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalKeyDown);
  document.removeEventListener('keyup', handleGlobalKeyUp);
  document.removeEventListener('click', handleClickOutside);
  document.removeEventListener('keydown', handleEscape);
  window.removeEventListener('scroll', handleScroll, true);
  window.removeEventListener('resize', handleResize);
});
</script>
