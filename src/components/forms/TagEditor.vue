<template>
  <div :class="ui.root()">
    <div v-if="label" :class="ui.labelRow()">
      <DocumentIcon :class="ui.labelIcon()" />
      <label :class="ui.label()">{{ label }}</label>
    </div>

    <p v-if="description" :class="ui.description()">
      {{ description }}
    </p>

    <div :class="ui.box()">
      <div ref="scrollContainer" :class="ui.scroll()">
        <div v-if="modelValue.length === 0" :class="ui.empty()">
          No tags added yet
        </div>
        <div v-else :class="ui.grid()">
          <div v-for="(tag, index) in modelValue" :key="index" :class="ui.chip()">
            <span :class="ui.chipText()">{{ tag }}</span>
            <button
              type="button"
              :class="ui.removeBtn()"
              title="Remove"
              @click="removeTag(index)"
            >
              <XIcon :class="ui.removeIcon()" />
            </button>
          </div>
        </div>
      </div>

      <div :class="ui.footer()">
        <div :class="ui.addRow()">
          <input
            v-model="newTag"
            type="text"
            :placeholder="placeholder || 'Add new tag...'"
            :class="ui.input()"
            @keydown.enter="addTag"
            @keydown.escape="clearInput"
          />
          <CustomButton type="button" size="sm" @click="addTag">
            <AddIcon :class="ui.addIcon()" />
          </CustomButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue';
import { tv } from '@/lib/tv';
import AddIcon from '../icons/AddIcon.vue';
import XIcon from '../icons/XIcon.vue';
import DocumentIcon from '../icons/DocumentIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

defineOptions({ name: 'TagEditor' });

const props = defineProps<{
  modelValue: string[];
  label?: string;
  description?: string;
  placeholder?: string;
  id?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string[]): void;
}>();

const newTag = ref('');
const scrollContainer = ref<HTMLElement | null>(null);

const ui = tv({
  slots: {
    root: 'space-y-2',
    labelRow: 'flex items-center gap-1.5',
    labelIcon: 'w-4 h-4 text-slate-500',
    label: 'text-sm font-medium text-slate-700',
    description: 'text-xs text-slate-500',
    box: 'border border-slate-200 rounded-md bg-white',
    scroll: 'max-h-28 overflow-y-auto p-2.5',
    empty: 'text-sm text-slate-400 italic',
    grid: 'grid grid-cols-4 gap-1.5',
    chip: 'flex items-center justify-between bg-slate-50 rounded-md px-2 py-1 min-w-0',
    chipText: 'text-xs text-slate-700 truncate flex-1 mr-1',
    removeBtn:
      'flex-shrink-0 w-6 h-6 text-red-600 hover:text-red-800 hover:bg-red-50 rounded-md transition-colors flex items-center justify-center cursor-pointer',
    removeIcon: 'w-3.5 h-3.5',
    footer: 'border-t border-slate-200 p-2.5',
    addRow: 'flex gap-1.5',
    input: [
      'flex-1 px-2.5 h-8 text-sm border border-slate-300 rounded-md',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    addIcon: 'w-4 h-4',
  },
})();

const addTag = () => {
  const tag = newTag.value.trim();
  if (tag && !props.modelValue.includes(tag)) {
    emit('update:modelValue', [...props.modelValue, tag]);
    newTag.value = '';
    nextTick(() => {
      if (scrollContainer.value) {
        scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
      }
    });
  }
};

const removeTag = (index: number) => {
  emit(
    'update:modelValue',
    props.modelValue.filter((_, i) => i !== index),
  );
};

const clearInput = () => {
  newTag.value = '';
};
</script>
