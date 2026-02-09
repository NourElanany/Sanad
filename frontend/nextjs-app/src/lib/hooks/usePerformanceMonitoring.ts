import { useEffect, useRef, useState } from 'react';

interface PerformanceMetrics {
  fps: number;
  loadTime: number;
  renderTime: number;
  memoryUsage?: number;
}

interface PerformanceMonitoringOptions {
  enabled?: boolean;
  logToConsole?: boolean;
  onMetricsUpdate?: (metrics: PerformanceMetrics) => void;
}

/**
 * Hook for monitoring performance metrics in React components
 * Tracks FPS, load time, render time, and memory usage
 */
export function usePerformanceMonitoring(
  componentName: string,
  options: PerformanceMonitoringOptions = {}
) {
  const {
    enabled = true,
    logToConsole = process.env.NODE_ENV === 'development',
    onMetricsUpdate,
  } = options;

  const [metrics, setMetrics] = useState<PerformanceMetrics>({
    fps: 60,
    loadTime: 0,
    renderTime: 0,
  });

  const frameCountRef = useRef(0);
  const lastFrameTimeRef = useRef(performance.now());
  const renderStartTimeRef = useRef(performance.now());
  const animationFrameRef = useRef<number>();

  useEffect(() => {
    if (!enabled) return;

    const startTime = performance.now();
    renderStartTimeRef.current = startTime;

    // Measure FPS
    const measureFPS = () => {
      const now = performance.now();
      frameCountRef.current++;

      const elapsed = now - lastFrameTimeRef.current;
      if (elapsed >= 1000) {
        const fps = Math.round((frameCountRef.current * 1000) / elapsed);
        frameCountRef.current = 0;
        lastFrameTimeRef.current = now;

        setMetrics((prev) => ({ ...prev, fps }));

        if (logToConsole && fps < 55) {
          console.warn(`[${componentName}] Low FPS detected: ${fps}`);
        }
      }

      animationFrameRef.current = requestAnimationFrame(measureFPS);
    };

    animationFrameRef.current = requestAnimationFrame(measureFPS);

    // Measure render time
    const renderTime = performance.now() - startTime;
    setMetrics((prev) => ({ ...prev, renderTime }));

    if (logToConsole && renderTime > 16.67) {
      console.warn(
        `[${componentName}] Slow render detected: ${renderTime.toFixed(2)}ms`
      );
    }

    // Measure memory usage (if available)
    if ('memory' in performance) {
      const memory = (performance as any).memory;
      const memoryUsage = Math.round(
        memory.usedJSHeapSize / 1024 / 1024
      );
      setMetrics((prev) => ({ ...prev, memoryUsage }));
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [enabled, componentName, logToConsole]);

  useEffect(() => {
    if (onMetricsUpdate) {
      onMetricsUpdate(metrics);
    }
  }, [metrics, onMetricsUpdate]);

  return metrics;
}

/**
 * Hook for measuring component render performance
 */
export function useRenderPerformance(componentName: string) {
  const renderCountRef = useRef(0);
  const renderTimesRef = useRef<number[]>([]);

  useEffect(() => {
    const startTime = performance.now();
    renderCountRef.current++;

    return () => {
      const renderTime = performance.now() - startTime;
      renderTimesRef.current.push(renderTime);

      // Keep only last 10 render times
      if (renderTimesRef.current.length > 10) {
        renderTimesRef.current.shift();
      }

      const avgRenderTime =
        renderTimesRef.current.reduce((a, b) => a + b, 0) /
        renderTimesRef.current.length;

      if (process.env.NODE_ENV === 'development') {
        console.log(
          `[${componentName}] Render #${renderCountRef.current}: ${renderTime.toFixed(2)}ms (avg: ${avgRenderTime.toFixed(2)}ms)`
        );
      }
    };
  });

  return {
    renderCount: renderCountRef.current,
    averageRenderTime:
      renderTimesRef.current.length > 0
        ? renderTimesRef.current.reduce((a, b) => a + b, 0) /
          renderTimesRef.current.length
        : 0,
  };
}

/**
 * Utility to measure async operation performance
 */
export async function measureAsyncOperation<T>(
  operationName: string,
  operation: () => Promise<T>
): Promise<T> {
  const startTime = performance.now();

  try {
    const result = await operation();
    const duration = performance.now() - startTime;

    if (process.env.NODE_ENV === 'development') {
      console.log(`[${operationName}] completed in ${duration.toFixed(2)}ms`);
    }

    if (duration > 1000) {
      console.warn(
        `[${operationName}] took longer than expected: ${duration.toFixed(2)}ms`
      );
    }

    return result;
  } catch (error) {
    const duration = performance.now() - startTime;
    console.error(
      `[${operationName}] failed after ${duration.toFixed(2)}ms:`,
      error
    );
    throw error;
  }
}

/**
 * Report Web Vitals to analytics
 */
export function reportWebVitals(metric: any) {
  if (process.env.NODE_ENV === 'development') {
    console.log('Web Vital:', metric);
  }

  // Send to analytics service
  // Example: analytics.track('web-vital', metric);
}
