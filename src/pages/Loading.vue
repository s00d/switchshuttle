<template>
  <div :class="ui.root()">
    <div :class="ui.spinner()" />
    <div :class="ui.title()">Loading...</div>
    <div :class="ui.subtitle()">
      Please wait while we prepare your application
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { tv } from '@/lib/tv';

defineOptions({ name: 'Loading' });

const loadingTv = tv({
  slots: {
    root: 'flex flex-col items-center justify-center h-screen w-screen bg-slate-50',
    spinner:
      'w-12 h-12 border-4 border-slate-200 border-t-blue-500 rounded-full animate-spin mb-4',
    title: 'text-lg text-slate-700 font-medium',
    subtitle: 'text-xs text-slate-500 mt-1.5',
  },
});

const ui = loadingTv();

onMounted(() => {
  // Add any initialization logic if needed
});
</script>
