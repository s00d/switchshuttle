<template>
  <div ref="rootEl" :class="ui.root()">
    <div
      v-if="!isEditing"
      :class="ui.trigger()"
      @click.stop="toggleDropdown"
    >
      <div :class="ui.triggerInner()">
        <span :class="ui.value()">{{ selectedLabel }}</span>
        <ChevronDownIcon :class="ui.chevron()" />
      </div>
    </div>

    <input
      v-else
      ref="customInput"
      v-model="customValue"
      :class="ui.customInput()"
      :placeholder="placeholder"
      @blur="finishEditing"
      @keyup.enter="finishEditing"
      @keyup.esc="cancelEditing"
      @click.stop
    />

    <div v-if="isOpen && !isEditing" :class="ui.menu()" @click.stop>
      <div :class="ui.menuInner()">
        <div
          v-for="option in options"
          :key="option.value"
          :class="
            ui.option({
              class:
                modelValue === option.value ? ui.optionActive() : undefined,
            })
          "
          @click="selectOption(option)"
        >
          {{ option.label }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import { tv } from '@/lib/tv';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';

defineOptions({ name: 'SchedulerSelect' });

interface Option {
  value: string;
  label: string;
}

interface Props {
  modelValue: string;
  options: Option[];
  placeholder?: string;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Enter value',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const isOpen = ref(false);
const isEditing = ref(false);
const customValue = ref('');
const customInput = ref<HTMLInputElement>();
const rootEl = ref<HTMLElement | null>(null);

const schedulerSelectTv = tv({
  slots: {
    root: 'relative',
    trigger: [
      'w-full px-2 py-1 text-xs border border-slate-300 rounded-md bg-white cursor-pointer',
      'hover:border-slate-400 transition-colors',
    ],
    triggerInner: 'flex items-center justify-between',
    value: 'text-slate-700 truncate',
    chevron: 'w-3 h-3 text-slate-400 transition-transform flex-shrink-0 ml-1',
    customInput: [
      'w-full px-2 py-1 text-xs border border-blue-500 rounded-md bg-white',
      'focus:outline-none focus:ring-1 focus:ring-blue-500',
    ],
    menu: 'absolute z-50 w-full mt-1 bg-white border border-slate-200 rounded-md shadow-lg max-h-48 overflow-auto',
    menuInner: 'py-1',
    option: 'px-2 py-1 text-xs cursor-pointer hover:bg-slate-50 transition-colors',
    optionActive: 'bg-blue-50 text-blue-700',
  },
  variants: {
    isOpen: {
      true: {
        trigger: 'border-blue-500 ring-1 ring-blue-500',
        chevron: 'rotate-180',
      },
      false: {},
    },
  },
});

const ui = computed(() => schedulerSelectTv({ isOpen: isOpen.value }));

const selectedLabel = computed(() => {
  const selected = props.options.find(
    (option) => option.value === props.modelValue,
  );
  if (selected) return selected.label;
  return props.modelValue || '';
});

const toggleDropdown = () => {
  isOpen.value = !isOpen.value;
};

const selectOption = (option: Option) => {
  if (option.value === 'custom') {
    isEditing.value = true;
    customValue.value = '';
    isOpen.value = false;
    nextTick(() => customInput.value?.focus());
  } else {
    emit('update:modelValue', option.value);
    isOpen.value = false;
  }
};

const finishEditing = () => {
  if (customValue.value.trim()) {
    emit('update:modelValue', customValue.value.trim());
  } else {
    const currentValue = props.modelValue;
    const hasValidValue = props.options.some(
      (option) => option.value === currentValue,
    );
    if (!hasValidValue && currentValue) {
      emit('update:modelValue', currentValue);
    }
  }
  isEditing.value = false;
  customValue.value = '';
};

const cancelEditing = () => {
  isEditing.value = false;
  customValue.value = '';
};

const closeDropdown = (event: Event) => {
  const target = event.target as Node;
  if (rootEl.value && !rootEl.value.contains(target)) {
    isOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', closeDropdown);
});

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown);
});
</script>
