<template>
  <div
    id="app"
    class="h-screen w-screen bg-slate-50 text-sm font-sans antialiased"
  >
    <main class="h-full overflow-auto">
      <router-view />
    </main>

    <!-- Performance Monitor (only in development) -->
    <PerformanceMonitor
      v-if="isDevelopment"
      :enabled="showPerformanceMonitor"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import PerformanceMonitor from './components/performance/PerformanceMonitor.vue';

const isDevelopment = import.meta.env.DEV;
const showPerformanceMonitor = ref(false);

// Toggle performance monitor with Ctrl+Shift+P
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
