<template>
  <div
    v-if="showMonitor"
    class="fixed bottom-4 right-4 bg-white border border-slate-200 rounded-lg shadow-lg p-4 w-80 z-50"
  >
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-semibold text-slate-900">Performance Monitor</h3>
      <CustomButton
        class="text-slate-400 hover:text-slate-600"
        title="Toggle monitor"
        @click="toggleMonitor"
      >
        <XIcon class="w-4 h-4" />
      </CustomButton>
    </div>

    <div class="space-y-2">
      <!-- Memory Usage -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-slate-600">Memory:</span>
        <div class="flex items-center space-x-2">
          <div class="w-16 h-2 bg-slate-200 rounded-full overflow-hidden">
            <div
              :class="memoryColorClass"
              class="h-full transition-all duration-300"
              :style="{ width: `${memoryUsage * 100}%` }"
            ></div>
          </div>
          <span class="text-xs font-mono text-slate-700"
            >{{ (memoryUsage * 100).toFixed(1) }}%</span
          >
        </div>
      </div>

      <!-- Render Time -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-slate-600">Render:</span>
        <span class="text-xs font-mono text-slate-700"
          >{{ lastRenderTime.toFixed(2) }}ms</span
        >
      </div>

      <!-- Performance Status -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-slate-600">Status:</span>
        <span
          :class="statusColorClass"
          class="text-xs font-medium px-2 py-1 rounded"
        >
          {{ performanceStatus }}
        </span>
      </div>

      <!-- Metrics Count -->
      <div class="flex items-center justify-between">
        <span class="text-xs text-slate-600">Metrics:</span>
        <span class="text-xs font-mono text-slate-700">{{ metricsCount }}</span>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex space-x-2 mt-3 pt-3 border-t border-slate-200">
      <CustomButton
        variant="secondary"
        size="sm"
        @click="clearMetrics"
      >
        Clear
      </CustomButton>
      <CustomButton
        variant="primary"
        size="sm"
        @click="exportMetrics"
      >
        Export
      </CustomButton>
      <CustomButton
        :variant="isMonitoring ? 'danger' : 'success'"
        size="sm"
        @click="toggleMonitoring"
      >
        {{ isMonitoring ? 'Stop' : 'Start' }}
      </CustomButton>
    </div>
  </div>

  <!-- Toggle Button -->
  <CustomButton
    v-else
    variant="primary"
    size="sm"
    class="fixed bottom-4 right-4 p-2 rounded-full shadow-lg z-50"
    title="Show performance monitor"
    @click="toggleMonitor"
  >
    <ChartIcon class="w-4 h-4" />
  </CustomButton>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { usePerformance } from '../../composables/usePerformance';
import XIcon from '../icons/XIcon.vue';
import ChartIcon from '../icons/ChartIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

const props = defineProps<{
  enabled?: boolean;
}>();

const showMonitor = ref(false);
const isMonitoring = ref(false);
const lastRenderTime = ref(0);

const {
  metrics,
  getMemoryUsage,
  addMetric,
  startMonitoring,
  stopMonitoring,
  clearMetrics,
  checkPerformance,
} = usePerformance();

// Computed properties
const memoryUsage = computed(() => getMemoryUsage());
const metricsCount = computed(() => metrics.length);
const performanceStatus = computed(() => checkPerformance());

const memoryColorClass = computed(() => {
  const usage = memoryUsage.value;
  if (usage > 0.8) return 'bg-red-500';
  if (usage > 0.6) return 'bg-yellow-500';
  return 'bg-green-500';
});

const statusColorClass = computed(() => {
  const status = performanceStatus.value;
  if (status === 'poor') return 'bg-red-100 text-red-800';
  if (status === 'warning') return 'bg-yellow-100 text-yellow-800';
  return 'bg-green-100 text-green-800';
});

const monitoringButtonClass = computed(() => {
  return isMonitoring.value
    ? 'bg-red-100 hover:bg-red-200 text-red-700'
    : 'bg-green-100 hover:bg-green-200 text-green-700';
});

// Methods
const toggleMonitor = () => {
  showMonitor.value = !showMonitor.value;
};

const toggleMonitoring = () => {
  if (isMonitoring.value) {
    stopMonitoring();
    isMonitoring.value = false;
  } else {
    startMonitoring();
    isMonitoring.value = true;
  }
};

const exportMetrics = () => {
  const data = {
    metrics: metrics,
    timestamp: new Date().toISOString(),
    summary: {
      averageMemory:
        metrics.reduce((sum: number, m: any) => sum + m.memoryUsage, 0) /
        metrics.length,
      averageRenderTime:
        metrics.reduce((sum: number, m: any) => sum + m.renderTime, 0) /
        metrics.length,
      performanceStatus: checkPerformance(),
    },
  };

  const blob = new Blob([JSON.stringify(data, null, 2)], {
    type: 'application/json',
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `performance-metrics-${new Date().toISOString().slice(0, 19)}.json`;
  a.click();
  URL.revokeObjectURL(url);
};

// Measure render time
const measureRender = () => {
  const start = performance.now();
  requestAnimationFrame(() => {
    const end = performance.now();
    lastRenderTime.value = end - start;
    addMetric({ renderTime: lastRenderTime.value });
  });
};

// Lifecycle
onMounted(() => {
  if (props.enabled) {
    showMonitor.value = true;
  }

  // Start monitoring if enabled
  if (props.enabled) {
    startMonitoring();
    isMonitoring.value = true;
  }

  // Measure initial render
  measureRender();
});

onUnmounted(() => {
  if (isMonitoring.value) {
    stopMonitoring();
  }
});

// Expose methods for parent components
defineExpose({
  toggleMonitor,
  toggleMonitoring,
  clearMetrics,
  exportMetrics,
  measureRender,
});
</script>
