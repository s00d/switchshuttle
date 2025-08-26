import { ref } from 'vue';

export interface PerformanceMetrics {
  memoryUsage: number;
  cpuUsage: number;
  renderTime: number;
  timestamp: Date;
}

export function usePerformance() {
  const metrics = ref<PerformanceMetrics[]>([]);
  const isMonitoring = ref(false);
  const maxMetrics = 100; // Maximum number of metrics to store

  // Measure render time
  const measureRenderTime = (callback: () => void): number => {
    const start = performance.now();
    callback();
    const end = performance.now();
    return end - start;
  };

  // Get memory information
  const getMemoryUsage = (): number => {
    if ('memory' in performance) {
      const memory = (performance as any).memory;
      return memory.usedJSHeapSize / memory.jsHeapSizeLimit;
    }
    return 0;
  };

  // Add metric
  const addMetric = (metric: Partial<PerformanceMetrics>) => {
    const fullMetric: PerformanceMetrics = {
      memoryUsage: metric.memoryUsage ?? getMemoryUsage(),
      cpuUsage: metric.cpuUsage ?? 0,
      renderTime: metric.renderTime ?? 0,
      timestamp: metric.timestamp ?? new Date(),
    };

    metrics.value.push(fullMetric);

    // Limit the number of metrics
    if (metrics.value.length > maxMetrics) {
      metrics.value = metrics.value.slice(-maxMetrics);
    }
  };

  // Start monitoring
  const startMonitoring = () => {
    isMonitoring.value = true;
    const interval = setInterval(() => {
      if (isMonitoring.value) {
        addMetric({
          memoryUsage: getMemoryUsage(),
          renderTime: 0,
        });
      } else {
        clearInterval(interval);
      }
    }, 1000); // Update every second

    return () => clearInterval(interval);
  };

  // Stop monitoring
  const stopMonitoring = () => {
    isMonitoring.value = false;
  };

  // Get average metrics
  const getAverageMetrics = (): PerformanceMetrics | null => {
    if (metrics.value.length === 0) return null;

    const sum = metrics.value.reduce(
      (acc, metric) => ({
        memoryUsage: acc.memoryUsage + metric.memoryUsage,
        cpuUsage: acc.cpuUsage + metric.cpuUsage,
        renderTime: acc.renderTime + metric.renderTime,
        timestamp: new Date(),
      }),
      { memoryUsage: 0, cpuUsage: 0, renderTime: 0, timestamp: new Date() }
    );

    const count = metrics.value.length;
    return {
      memoryUsage: sum.memoryUsage / count,
      cpuUsage: sum.cpuUsage / count,
      renderTime: sum.renderTime / count,
      timestamp: sum.timestamp,
    };
  };

  // Clear metrics
  const clearMetrics = () => {
    metrics.value = [];
  };

  // Check performance
  const checkPerformance = (): 'good' | 'warning' | 'poor' => {
    const average = getAverageMetrics();
    if (!average) return 'good';

    if (average.memoryUsage > 0.8 || average.renderTime > 100) {
      return 'poor';
    } else if (average.memoryUsage > 0.6 || average.renderTime > 50) {
      return 'warning';
    }

    return 'good';
  };

  // Debounce function
  const debounce = <T extends (...args: any[]) => any>(
    func: T,
    delay: number
  ): ((...args: Parameters<T>) => void) => {
    let timeoutId: number;
    return (...args: Parameters<T>) => {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => func(...args), delay);
    };
  };

  // Throttling function
  const throttle = <T extends (...args: any[]) => any>(
    func: T,
    delay: number
  ): ((...args: Parameters<T>) => void) => {
    let lastCall = 0;
    return (...args: Parameters<T>) => {
      const now = Date.now();
      if (now - lastCall >= delay) {
        lastCall = now;
        func(...args);
      }
    };
  };

  // Lazy loading
  const lazyLoad = <T>(loader: () => Promise<T>, fallback?: T): Promise<T> => {
    return new Promise(resolve => {
      // Use requestIdleCallback if available
      if ('requestIdleCallback' in window) {
        (window as any).requestIdleCallback(() => {
          loader()
            .then(resolve)
            .catch(() => {
              if (fallback !== undefined) resolve(fallback);
            });
        });
      } else {
        // Fallback for browsers without requestIdleCallback
        setTimeout(() => {
          loader()
            .then(resolve)
            .catch(() => {
              if (fallback !== undefined) resolve(fallback);
            });
        }, 0);
      }
    });
  };

  return {
    metrics: metrics.value,
    isMonitoring: isMonitoring.value,
    measureRenderTime,
    getMemoryUsage,
    addMetric,
    startMonitoring,
    stopMonitoring,
    getAverageMetrics,
    clearMetrics,
    checkPerformance,
    debounce,
    throttle,
    lazyLoad,
  };
}
