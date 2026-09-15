<template>
  <div v-if="showMonitor" :class="ui.panel()">
    <div :class="ui.header()">
      <h3 :class="ui.title()">Performance Monitor</h3>
      <CustomButton title="Toggle monitor" @click="toggleMonitor">
        <XIcon :class="ui.icon()" />
      </CustomButton>
    </div>

    <div :class="ui.metrics()">
      <div :class="ui.row()">
        <span :class="ui.label()">Memory:</span>
        <div :class="ui.memoryWrap()">
          <div :class="ui.barTrack()">
            <div
              :class="ui.barFill({ tone: memoryTone })"
              :style="{ width: `${memoryUsage * 100}%` }"
            />
          </div>
          <span :class="ui.value()"
            >{{ (memoryUsage * 100).toFixed(1) }}%</span
          >
        </div>
      </div>

      <div :class="ui.row()">
        <span :class="ui.label()">Render:</span>
        <span :class="ui.value()">{{ lastRenderTime.toFixed(2) }}ms</span>
      </div>

      <div :class="ui.row()">
        <span :class="ui.label()">Status:</span>
        <span :class="ui.status({ status: performanceStatus })">
          {{ performanceStatus }}
        </span>
      </div>

      <div :class="ui.row()">
        <span :class="ui.label()">Metrics:</span>
        <span :class="ui.value()">{{ metricsCount }}</span>
      </div>
    </div>

    <div :class="ui.actions()">
      <CustomButton variant="secondary" size="sm" @click="clearMetrics">
        Clear
      </CustomButton>
      <CustomButton variant="primary" size="sm" @click="exportMetrics">
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

  <CustomButton
    v-else
    variant="primary"
    size="sm"
    :class="ui.fab()"
    title="Show performance monitor"
    @click="toggleMonitor"
  >
    <ChartIcon :class="ui.icon()" />
  </CustomButton>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { tv } from '@/lib/tv';
import { usePerformance } from '../../composables/usePerformance';
import XIcon from '../icons/XIcon.vue';
import ChartIcon from '../icons/ChartIcon.vue';
import CustomButton from '../ui/CustomButton.vue';

defineOptions({ name: 'PerformanceMonitor' });

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

const memoryUsage = computed(() => getMemoryUsage());
const metricsCount = computed(() => metrics.length);
const performanceStatus = computed(() => checkPerformance());

const memoryTone = computed(() => {
  const usage = memoryUsage.value;
  if (usage > 0.8) return 'danger' as const;
  if (usage > 0.6) return 'warning' as const;
  return 'ok' as const;
});

const performanceMonitorTv = tv({
  slots: {
    panel:
      'fixed bottom-4 right-4 bg-white border border-slate-200 rounded-md shadow-lg p-3 w-72 z-50',
    header: 'flex items-center justify-between mb-2',
    title: 'text-sm font-semibold text-slate-900',
    icon: 'w-4 h-4',
    metrics: 'space-y-1.5',
    row: 'flex items-center justify-between',
    label: 'text-xs text-slate-600',
    memoryWrap: 'flex items-center gap-2',
    barTrack: 'w-16 h-1.5 bg-slate-200 rounded-full overflow-hidden',
    barFill: 'h-full transition-all duration-300',
    value: 'text-xs font-mono text-slate-700',
    status: 'text-xs font-medium px-1.5 py-0.5 rounded-md',
    actions: 'flex gap-1.5 mt-2.5 pt-2.5 border-t border-slate-200',
    fab: 'fixed bottom-4 right-4 !p-2 !rounded-full shadow-lg z-50',
  },
  variants: {
    tone: {
      danger: { barFill: 'bg-red-500' },
      warning: { barFill: 'bg-yellow-500' },
      ok: { barFill: 'bg-green-500' },
    },
    status: {
      poor: { status: 'bg-red-100 text-red-800' },
      warning: { status: 'bg-yellow-100 text-yellow-800' },
      good: { status: 'bg-green-100 text-green-800' },
    },
  },
});

const ui = performanceMonitorTv();

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

const measureRender = () => {
  const start = performance.now();
  requestAnimationFrame(() => {
    const end = performance.now();
    lastRenderTime.value = end - start;
    addMetric({ renderTime: lastRenderTime.value });
  });
};

onMounted(() => {
  if (props.enabled) {
    showMonitor.value = true;
  }

  if (props.enabled) {
    startMonitoring();
    isMonitoring.value = true;
  }

  measureRender();
});

onUnmounted(() => {
  if (isMonitoring.value) {
    stopMonitoring();
  }
});

defineExpose({
  toggleMonitor,
  toggleMonitoring,
  clearMetrics,
  exportMetrics,
  measureRender,
});
</script>
