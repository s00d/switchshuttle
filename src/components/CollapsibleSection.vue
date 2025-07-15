<template>
  <div class="space-y-4">
    <button 
      @click="toggleCollapsed" 
      type="button"
      class="flex items-center justify-between w-full p-3 text-left bg-slate-50 hover:bg-slate-100 rounded-lg border border-slate-200 transition-colors duration-200"
    >
      <div class="flex items-center space-x-2">
        <ChevronRightIcon :collapsed="!collapsed" />
        <span class="font-medium text-slate-700">{{ title }}</span>
      </div>
      <div class="text-sm text-slate-500">
        {{ summary }}
      </div>
    </button>
    
    <div v-if="!collapsed" class="space-y-6 p-4 bg-slate-50 rounded-lg border border-slate-200">
      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import ChevronRightIcon from './icons/ChevronRightIcon.vue';

interface Props {
  title: string;
  summary: string;
  defaultCollapsed?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  defaultCollapsed: true
});

const collapsed = ref(props.defaultCollapsed);

const toggleCollapsed = () => {
  collapsed.value = !collapsed.value;
};
</script> 