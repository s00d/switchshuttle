<template>
  <div ref="rootEl" :class="ui.root()">
    <button
      type="button"
      :class="ui.trigger()"
      @click="toggleDropdown"
    >
      <div :class="ui.triggerInner()">
        <div :class="ui.valueRow()">
          <span v-if="selectedOption?.icon" :class="ui.icon()">{{
            selectedOption.icon
          }}</span>
          <div :class="ui.valueText()">
            <div :class="ui.label()">
              {{ selectedOption?.label || placeholder }}
            </div>
            <div v-if="selectedOption?.description" :class="ui.description()">
              {{ selectedOption.description }}
            </div>
          </div>
        </div>
        <ChevronDownIcon :class="ui.chevron()" />
      </div>
    </button>

    <div v-if="isOpen" :class="ui.menu()">
      <div :class="ui.menuInner()">
        <div
          v-for="option in options"
          :key="option.value"
          :class="
            ui.option({
              class:
                option.value === modelValue ? ui.optionActive() : undefined,
            })
          "
          @click="selectOption(option)"
        >
          <span v-if="option.icon" :class="ui.optionIcon()">{{
            option.icon
          }}</span>
          <div :class="ui.valueText()">
            <div :class="ui.optionLabel()">{{ option.label }}</div>
            <div v-if="option.description" :class="ui.optionDescription()">
              {{ option.description }}
            </div>
          </div>
          <CheckIcon
            v-if="option.value === modelValue"
            :class="ui.check()"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { tv } from '@/lib/tv';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import CheckIcon from '../icons/CheckIcon.vue';

defineOptions({ name: 'CustomSelect' });

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
const rootEl = ref<HTMLElement | null>(null);

const selectTv = tv({
  slots: {
    root: 'relative',
    trigger: [
      'w-full px-2.5 h-8 text-left bg-white border border-slate-300 text-sm rounded-md',
      'transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    triggerInner: 'flex items-center justify-between h-full',
    valueRow: 'flex items-center gap-2 min-w-0 flex-1',
    icon: 'text-base flex-shrink-0',
    valueText: 'min-w-0 flex-1',
    label: 'text-slate-900 truncate text-sm',
    description: 'text-xs text-slate-500 truncate',
    chevron:
      'w-4 h-4 text-slate-400 transition-transform duration-200 flex-shrink-0 ml-2',
    menu: 'absolute z-50 w-full mt-1 bg-white border border-slate-200 rounded-md shadow-lg max-h-60 overflow-auto',
    menuInner: 'py-1',
    option:
      'flex items-start gap-2.5 px-2.5 py-1.5 cursor-pointer hover:bg-slate-50 transition-colors text-sm',
    optionActive: 'bg-blue-50 text-blue-700',
    optionIcon: 'text-base flex-shrink-0 mt-0.5',
    optionLabel: 'font-medium',
    optionDescription: 'text-xs text-slate-500 mt-0.5',
    check: 'w-4 h-4 text-blue-600 flex-shrink-0 mt-0.5',
  },
  variants: {
    isOpen: {
      true: {
        trigger: 'ring-2 ring-blue-500 border-blue-500',
        chevron: 'rotate-180',
      },
      false: {},
    },
  },
});

const ui = computed(() => selectTv({ isOpen: isOpen.value }));

const selectedOption = computed(() =>
  props.options.find((option) => option.value === props.modelValue),
);

function toggleDropdown() {
  isOpen.value = !isOpen.value;
}

function selectOption(option: Option) {
  emit('update:modelValue', option.value);
  isOpen.value = false;
}

function handleClickOutside(event: Event) {
  const target = event.target as Node;
  if (rootEl.value && !rootEl.value.contains(target)) {
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
