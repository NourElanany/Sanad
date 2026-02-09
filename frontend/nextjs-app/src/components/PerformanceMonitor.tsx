/**
 * Performance Monitor Component
 * 
 * Real-time performance monitoring dashboard for development
 * Shows FPS, memory usage, bundle sizes, and Web Vitals
 */

'use client';

import { useEffect, useState } from 'react';
import { getBundleMetrics, analyzeBundlePerformance } from '@/lib/utils/code-splitting';
import { getNetworkInfo } from '@/lib/utils/lazy-loading';

interface PerformanceMetrics {
  fps: number;
  memory?: {
    used: number;
    total: number;
    limit: number;
  };
  webVitals: {
    FCP?: number;
    LCP?: number;
    FID?: number;
    CLS?: number;
    TTFB?: number;
  };
  network: {
    effectiveType?: string;
    downlink?: number;
    rtt?: number;
    saveData?: boolean;
  };
  bundleStats?: {
    totalSize: number;
    totalLoadTime: number;
    averageLoadTime: number;
  };
}

export default function PerformanceMonitor() {
  const [isVisible, setIsVisible] = useState(false);
  const [metrics, setMetrics] = useState<PerformanceMetrics>({
    fps: 0,
    webVitals: {},
    network: {},
  });

  useEffect(() => {
    // Only show in development
    if (process.env.NODE_ENV !== 'development') {
      return;
    }

    // Toggle visibility with Ctrl+Shift+P
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.shiftKey && e.key === 'P') {
        setIsVisible(prev => !prev);
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, []);

  useEffect(() => {
    if (!isVisible) return;

    // FPS monitoring
    let frameCount = 0;
    let lastTime = performance.now();
    let animationFrameId: number;

    const measureFPS = () => {
      frameCount++;
      const currentTime = performance.now();
      
      if (currentTime >= lastTime + 1000) {
        const fps = Math.round((frameCount * 1000) / (currentTime - lastTime));
        setMetrics(prev => ({ ...prev, fps }));
        frameCount = 0;
        lastTime = currentTime;
      }
      
      animationFrameId = requestAnimationFrame(measureFPS);
    };

    animationFrameId = requestAnimationFrame(measureFPS);

    // Memory monitoring
    const updateMemory = () => {
      if ('memory' in performance) {
        const memory = (performance as any).memory;
        setMetrics(prev => ({
          ...prev,
          memory: {
            used: memory.usedJSHeapSize,
            total: memory.totalJSHeapSize,
            limit: memory.jsHeapSizeLimit,
          },
        }));
      }
    };

    const memoryInterval = setInterval(updateMemory, 1000);

    // Network info
    const updateNetwork = () => {
      setMetrics(prev => ({
        ...prev,
        network: getNetworkInfo(),
      }));
    };

    updateNetwork();
    const networkInterval = setInterval(updateNetwork, 5000);

    // Web Vitals
    if ('PerformanceObserver' in window) {
      try {
        // FCP
        const fcpObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          const fcp = entries[entries.length - 1];
          setMetrics(prev => ({
            ...prev,
            webVitals: { ...prev.webVitals, FCP: fcp.startTime },
          }));
        });
        fcpObserver.observe({ entryTypes: ['paint'] });

        // LCP
        const lcpObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          const lcp = entries[entries.length - 1];
          setMetrics(prev => ({
            ...prev,
            webVitals: { ...prev.webVitals, LCP: lcp.startTime },
          }));
        });
        lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });

        // FID
        const fidObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          entries.forEach((entry: any) => {
            setMetrics(prev => ({
              ...prev,
              webVitals: { ...prev.webVitals, FID: entry.processingStart - entry.startTime },
            }));
          });
        });
        fidObserver.observe({ entryTypes: ['first-input'] });

        // CLS
        let clsValue = 0;
        const clsObserver = new PerformanceObserver((list) => {
          for (const entry of list.getEntries()) {
            if (!(entry as any).hadRecentInput) {
              clsValue += (entry as any).value;
              setMetrics(prev => ({
                ...prev,
                webVitals: { ...prev.webVitals, CLS: clsValue },
              }));
            }
          }
        });
        clsObserver.observe({ entryTypes: ['layout-shift'] });
      } catch (error) {
        console.error('Failed to observe performance metrics:', error);
      }
    }

    // Bundle stats
    const updateBundleStats = () => {
      const stats = analyzeBundlePerformance();
      setMetrics(prev => ({
        ...prev,
        bundleStats: {
          totalSize: stats.totalSize,
          totalLoadTime: stats.totalLoadTime,
          averageLoadTime: stats.averageLoadTime,
        },
      }));
    };

    updateBundleStats();
    const bundleInterval = setInterval(updateBundleStats, 10000);

    return () => {
      cancelAnimationFrame(animationFrameId);
      clearInterval(memoryInterval);
      clearInterval(networkInterval);
      clearInterval(bundleInterval);
    };
  }, [isVisible]);

  if (!isVisible || process.env.NODE_ENV !== 'development') {
    return null;
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  };

  const getMetricColor = (value: number, thresholds: { good: number; poor: number }) => {
    if (value <= thresholds.good) return 'text-green-500';
    if (value <= thresholds.poor) return 'text-yellow-500';
    return 'text-red-500';
  };

  return (
    <div className="fixed bottom-4 right-4 z-50 w-96 bg-gray-900 text-white rounded-lg shadow-2xl p-4 font-mono text-xs">
      <div className="flex justify-between items-center mb-3 border-b border-gray-700 pb-2">
        <h3 className="font-bold text-sm">⚡ Performance Monitor</h3>
        <button
          onClick={() => setIsVisible(false)}
          className="text-gray-400 hover:text-white"
        >
          ✕
        </button>
      </div>

      {/* FPS */}
      <div className="mb-3">
        <div className="flex justify-between items-center">
          <span className="text-gray-400">FPS:</span>
          <span className={metrics.fps >= 55 ? 'text-green-500' : metrics.fps >= 30 ? 'text-yellow-500' : 'text-red-500'}>
            {metrics.fps}
          </span>
        </div>
        <div className="w-full bg-gray-700 h-1 rounded mt-1">
          <div
            className={`h-1 rounded transition-all ${
              metrics.fps >= 55 ? 'bg-green-500' : metrics.fps >= 30 ? 'bg-yellow-500' : 'bg-red-500'
            }`}
            style={{ width: `${(metrics.fps / 60) * 100}%` }}
          />
        </div>
      </div>

      {/* Memory */}
      {metrics.memory && (
        <div className="mb-3">
          <div className="flex justify-between items-center">
            <span className="text-gray-400">Memory:</span>
            <span>{formatBytes(metrics.memory.used)} / {formatBytes(metrics.memory.limit)}</span>
          </div>
          <div className="w-full bg-gray-700 h-1 rounded mt-1">
            <div
              className="bg-blue-500 h-1 rounded transition-all"
              style={{ width: `${(metrics.memory.used / metrics.memory.limit) * 100}%` }}
            />
          </div>
        </div>
      )}

      {/* Web Vitals */}
      <div className="mb-3 border-t border-gray-700 pt-2">
        <h4 className="text-gray-400 mb-2">Web Vitals:</h4>
        <div className="space-y-1">
          {metrics.webVitals.FCP && (
            <div className="flex justify-between">
              <span className="text-gray-400">FCP:</span>
              <span className={getMetricColor(metrics.webVitals.FCP, { good: 1800, poor: 3000 })}>
                {metrics.webVitals.FCP.toFixed(0)}ms
              </span>
            </div>
          )}
          {metrics.webVitals.LCP && (
            <div className="flex justify-between">
              <span className="text-gray-400">LCP:</span>
              <span className={getMetricColor(metrics.webVitals.LCP, { good: 2500, poor: 4000 })}>
                {metrics.webVitals.LCP.toFixed(0)}ms
              </span>
            </div>
          )}
          {metrics.webVitals.FID && (
            <div className="flex justify-between">
              <span className="text-gray-400">FID:</span>
              <span className={getMetricColor(metrics.webVitals.FID, { good: 100, poor: 300 })}>
                {metrics.webVitals.FID.toFixed(0)}ms
              </span>
            </div>
          )}
          {metrics.webVitals.CLS !== undefined && (
            <div className="flex justify-between">
              <span className="text-gray-400">CLS:</span>
              <span className={getMetricColor(metrics.webVitals.CLS, { good: 0.1, poor: 0.25 })}>
                {metrics.webVitals.CLS.toFixed(3)}
              </span>
            </div>
          )}
        </div>
      </div>

      {/* Network */}
      <div className="mb-3 border-t border-gray-700 pt-2">
        <h4 className="text-gray-400 mb-2">Network:</h4>
        <div className="space-y-1">
          {metrics.network.effectiveType && (
            <div className="flex justify-between">
              <span className="text-gray-400">Type:</span>
              <span>{metrics.network.effectiveType}</span>
            </div>
          )}
          {metrics.network.downlink && (
            <div className="flex justify-between">
              <span className="text-gray-400">Downlink:</span>
              <span>{metrics.network.downlink.toFixed(1)} Mbps</span>
            </div>
          )}
          {metrics.network.rtt && (
            <div className="flex justify-between">
              <span className="text-gray-400">RTT:</span>
              <span>{metrics.network.rtt}ms</span>
            </div>
          )}
          {metrics.network.saveData && (
            <div className="text-yellow-500">⚠️ Save Data Mode</div>
          )}
        </div>
      </div>

      {/* Bundle Stats */}
      {metrics.bundleStats && metrics.bundleStats.totalSize > 0 && (
        <div className="border-t border-gray-700 pt-2">
          <h4 className="text-gray-400 mb-2">Bundle Stats:</h4>
          <div className="space-y-1">
            <div className="flex justify-between">
              <span className="text-gray-400">Total Size:</span>
              <span>{formatBytes(metrics.bundleStats.totalSize)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Avg Load:</span>
              <span>{metrics.bundleStats.averageLoadTime.toFixed(0)}ms</span>
            </div>
          </div>
        </div>
      )}

      <div className="mt-3 text-gray-500 text-center border-t border-gray-700 pt-2">
        Press Ctrl+Shift+P to toggle
      </div>
    </div>
  );
}
