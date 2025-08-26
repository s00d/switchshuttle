<template>
  <div class="space-y-4">
    <CustomButton
      variant="ghost"
      class="flex items-center justify-between w-full p-3 text-left"
      @click="toggleCollapsed"
    >
      <div class="flex items-center space-x-2">
        <ChevronRightIcon :collapsed="!collapsed" />
        <span class="font-medium text-slate-700">{{ title }}</span>
      </div>
      <div class="text-sm text-slate-500">
        {{ summary }}
      </div>
    </CustomButton>

    <div
      v-if="!collapsed"
      class="space-y-6 p-4 bg-slate-50 rounded-lg border border-slate-200"
    >
      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import ChevronRightIcon from '../icons/ChevronRightIcon.vue';
import CustomButton from './CustomButton.vue';

interface Props {
  title: string;
  summary: string;
  defaultCollapsed?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  defaultCollapsed: true,
});

const collapsed = ref(props.defaultCollapsed);

const toggleCollapsed = () => {
  collapsed.value = !collapsed.value;
};
</script>
