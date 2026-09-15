<template>
  <div id="app" :class="ui.root()">
    <main :class="ui.main()">
      <router-view />
    </main>

    <PerformanceMonitor
      v-if="isDevelopment"
      :enabled="showPerformanceMonitor"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { tv } from '@/lib/tv';
import PerformanceMonitor from './components/performance/PerformanceMonitor.vue';

defineOptions({ name: 'App' });

const isDevelopment = import.meta.env.DEV;
const showPerformanceMonitor = ref(false);

const appTv = tv({
  slots: {
    root: 'h-screen w-screen bg-slate-50 text-sm font-sans antialiased',
    main: 'h-full overflow-auto',
  },
});

const ui = appTv();

onMounted(() => {
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.ctrlKey && event.shiftKey && event.key === 'P') {
      event.preventDefault();
      showPerformanceMonitor.value = !showPerformanceMonitor.value;
    }
  };

  document.addEventListener('keydown', handleKeydown);
});
</script>
