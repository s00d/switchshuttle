<template>
  <div class="space-y-3">
    <!-- Label -->
    <div v-if="label" class="flex items-center space-x-2">
      <DocumentIcon class="w-4 h-4 text-slate-500" />
      <label class="text-sm font-medium text-slate-700">{{ label }}</label>
    </div>

    <!-- Description -->
    <p v-if="description" class="text-xs text-slate-500">
      {{ description }}
    </p>

    <!-- Tags Container -->
    <div class="border border-slate-200 rounded-lg bg-white">
      <!-- Tags List with Scroll -->
      <div ref="scrollContainer" class="max-h-32 overflow-y-auto p-3">
        <div v-if="modelValue.length === 0" class="text-sm text-slate-400 italic">
          No tags added yet
        </div>
        <div v-else class="grid grid-cols-4 gap-2 overflow-x-auto">
          <div
            v-for="(tag, index) in modelValue"
            :key="index"
            class="flex items-center justify-between bg-slate-50 rounded px-3 py-2 min-w-0 flex-shrink-0"
          >
            <span class="text-sm text-slate-700 truncate flex-1 mr-2">{{ tag }}</span>
            <button
              type="button"
              class="flex-shrink-0 w-6 h-6 text-red-600 hover:text-red-800 hover:bg-red-100 rounded transition-colors flex items-center justify-center cursor-pointer"
              @click="removeTag(index)"
              title="Remove"
            >
              <XIcon class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <!-- Add Tag Input -->
      <div class="border-t border-slate-200 p-3">
        <div class="flex space-x-2">
          <input
            v-model="newTag"
            type="text"
            :placeholder="placeholder || 'Add new tag...'"
            class="flex-1 px-3 py-2 text-sm border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            @keydown.enter="addTag"
            @keydown.escape="clearInput"
          />
          <button
            type="button"
            class="px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
            @click="addTag"
          >
            <AddIcon class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue';
import AddIcon from '../icons/AddIcon.vue';
import XIcon from '../icons/XIcon.vue';
import DocumentIcon from '../icons/DocumentIcon.vue';

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

const addTag = () => {
  const tag = newTag.value.trim();
  if (tag && !props.modelValue.includes(tag)) {
    const updatedTags = [...props.modelValue, tag];
    emit('update:modelValue', updatedTags);
    newTag.value = '';
    
    // Restore scroll position after DOM update
    nextTick(() => {
      if (scrollContainer.value) {
        scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
      }
    });
  }
};

const removeTag = (index: number) => {
  const updatedTags = props.modelValue.filter((_, i) => i !== index);
  emit('update:modelValue', updatedTags);
};

const clearInput = () => {
  newTag.value = '';
};
</script> 