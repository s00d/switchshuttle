<template>
  <div :class="ui.root()">
    <label v-if="label" :class="ui.label()">
      {{ label }}
    </label>
    <div :class="ui.field()">
      <input
        ref="inputRef"
        :value="displayValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :class="ui.input()"
        readonly
        @focus="startRecording"
        @blur="stopRecording"
        @keydown="handleKeyDown"
        @keyup="handleKeyUp"
      />
      <div v-if="isRecording" :class="ui.recordingOverlay()">
        <span :class="ui.recordingText()">Press keys...</span>
      </div>
      <CustomButton
        v-if="modelValue"
        variant="ghost"
        :class="ui.clearBtn()"
        @click="clearHotkey"
      >
        <XIcon :class="ui.icon()" />
      </CustomButton>

      <CustomButton
        variant="ghost"
        :class="ui.dropdownToggle()"
        title="Select from available keys"
        @click="toggleDropdown"
      >
        <ChevronDownIcon :class="ui.icon()" />
      </CustomButton>

      <!-- Teleported Dropdown -->
      <Teleport to="body">
        <div
          v-if="showDropdown"
          :class="ui.dropdown()"
          :style="dropdownStyle"
        >
          <div :class="ui.dropdownBody()">
            <div :class="ui.sections()">
              <!-- Modifiers -->
              <div>
                <h4 :class="ui.sectionTitle()">Modifiers</h4>
                <div :class="ui.gridModifiers()">
                  <CustomButton
                    v-for="modifier in modifiers"
                    :key="modifier.value"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class: selectedModifiers.includes(modifier.value)
                          ? ui.keyBtnSelected()
                          : undefined,
                      })
                    "
                    @click="selectModifier(modifier.value)"
                  >
                    {{ modifier.label }}
                  </CustomButton>
                </div>
              </div>

              <!-- Letters -->
              <div>
                <h4 :class="ui.sectionTitle()">Letters</h4>
                <div :class="ui.gridLetters()">
                  <CustomButton
                    v-for="letter in letters"
                    :key="letter"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class:
                          selectedKey === letter
                            ? ui.keyBtnSelected()
                            : undefined,
                      })
                    "
                    @click="selectKey(letter)"
                  >
                    {{ letter.toUpperCase() }}
                  </CustomButton>
                </div>
              </div>

              <!-- Numbers -->
              <div>
                <h4 :class="ui.sectionTitle()">Numbers</h4>
                <div :class="ui.gridNumbers()">
                  <CustomButton
                    v-for="num in numbers"
                    :key="num"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class:
                          selectedKey === num
                            ? ui.keyBtnSelected()
                            : undefined,
                      })
                    "
                    @click="selectKey(num)"
                  >
                    {{ num }}
                  </CustomButton>
                </div>
              </div>

              <!-- Function Keys -->
              <div>
                <h4 :class="ui.sectionTitle()">Function Keys</h4>
                <div :class="ui.gridFunction()">
                  <CustomButton
                    v-for="fkey in functionKeys"
                    :key="fkey"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class:
                          selectedKey === fkey
                            ? ui.keyBtnSelected()
                            : undefined,
                      })
                    "
                    @click="selectKey(fkey)"
                  >
                    {{ fkey.toUpperCase() }}
                  </CustomButton>
                </div>
              </div>

              <!-- Special Keys -->
              <div>
                <h4 :class="ui.sectionTitle()">Special Keys</h4>
                <div :class="ui.gridSpecial()">
                  <CustomButton
                    v-for="special in specialKeys"
                    :key="special.value"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class: [
                          ui.keyBtnCompact(),
                          selectedKey === special.value
                            ? ui.keyBtnSelected()
                            : undefined,
                        ],
                      })
                    "
                    @click="selectKey(special.value)"
                  >
                    {{ special.label }}
                  </CustomButton>
                </div>
              </div>

              <!-- Navigation Keys -->
              <div>
                <h4 :class="ui.sectionTitle()">Navigation</h4>
                <div :class="ui.gridNav()">
                  <CustomButton
                    v-for="nav in navigationKeys"
                    :key="nav.value"
                    variant="ghost"
                    :class="
                      ui.keyBtn({
                        class:
                          selectedKey === nav.value
                            ? ui.keyBtnSelected()
                            : undefined,
                      })
                    "
                    @click="selectKey(nav.value)"
                  >
                    {{ nav.label }}
                  </CustomButton>
                </div>
              </div>
            </div>
          </div>

          <div :class="ui.footer()">
            <CustomButton
              :class="ui.applyBtn()"
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

    <p v-if="error" :class="ui.error()">{{ error }}</p>
    <p v-else-if="hint" :class="ui.hint()">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import { tv } from '@/lib/tv';
import XIcon from '../icons/XIcon.vue';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

defineOptions({ name: 'HotkeyInput' });

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

const hotkeyTv = tv({
  slots: {
    root: 'space-y-1',
    label: 'block text-sm font-medium text-slate-700',
    field: 'relative',
    input: [
      'w-full border border-slate-300 text-sm rounded-md transition-all duration-200',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
      'disabled:bg-slate-100 disabled:text-slate-500 disabled:cursor-not-allowed',
    ],
    recordingOverlay:
      'absolute inset-0 bg-blue-50 border-2 border-blue-500 rounded-md flex items-center justify-center',
    recordingText: 'text-blue-600 text-sm font-medium',
    clearBtn: 'absolute right-2 top-1/2 -translate-y-1/2 p-1',
    dropdownToggle: 'absolute right-8 top-1/2 -translate-y-1/2 p-1',
    icon: 'w-4 h-4',
    dropdown:
      'hotkey-dropdown bg-white border border-slate-200 rounded-md shadow-lg',
    dropdownBody: 'p-2.5 flex-1 overflow-y-auto max-h-[350px]',
    sections: 'space-y-2',
    sectionTitle:
      'text-xs font-semibold text-slate-700 uppercase tracking-wide mb-1.5',
    gridModifiers: 'grid grid-cols-2 gap-1',
    gridLetters: 'grid grid-cols-8 gap-1',
    gridNumbers: 'grid grid-cols-10 gap-1',
    gridFunction: 'grid grid-cols-6 gap-1',
    gridSpecial: 'grid grid-cols-3 gap-1',
    gridNav: 'grid grid-cols-4 gap-1',
    keyBtn: 'px-2 py-1 text-xs border border-slate-200 rounded-md',
    keyBtnSelected: 'bg-blue-100 text-blue-700 border-blue-300',
    keyBtnCompact: 'px-1 py-1',
    footer: 'p-2 border-t border-slate-200 bg-white rounded-b-md',
    applyBtn: 'w-full text-sm py-2',
    error: 'text-xs text-red-600',
    hint: 'text-xs text-slate-500',
  },
  variants: {
    size: {
      sm: { input: 'px-2.5 py-1.5 h-8' },
      md: { input: 'px-2.5 py-2 h-8' },
      lg: { input: 'px-3 py-2.5 h-9' },
    },
    error: {
      true: {
        input: 'border-red-500 focus:ring-red-500 focus:border-red-500',
      },
    },
    isRecording: {
      true: {
        input: 'ring-2 ring-blue-500 border-blue-500',
      },
    },
  },
  compoundVariants: [
    {
      error: true,
      isRecording: true,
      class: {
        input: 'border-red-500 ring-2 ring-red-500',
      },
    },
  ],
  defaultVariants: {
    size: 'md',
  },
});

const ui = computed(() =>
  hotkeyTv({
    size: props.size,
    error: !!props.error,
    isRecording: isRecording.value,
  }),
);

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
