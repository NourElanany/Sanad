/**
 * Code Splitting Utilities
 * 
 * Provides utilities for dynamic imports and code splitting
 * to optimize bundle size and loading performance.
 */

import { ComponentType } from 'react';
import { lazyWithRetry, LazyLoadOptions } from './lazy-loading';

/**
 * Route-based code splitting configuration
 */
export const RouteComponents = {
  // Dashboard
  Dashboard: () => lazyWithRetry(
    () => import('@/app/dashboard/page'),
    { preloadAfter: 2000 }
  ),
  
  // Quran
  QuranIndex: () => lazyWithRetry(
    () => import('@/app/quran/page'),
    { preloadAfter: 1000 }
  ),
  QuranMushaf: () => lazyWithRetry(
    () => import('@/app/quran/mushaf/[page]/page')
  ),
  
  // AI Assistant
  AIAssistant: () => lazyWithRetry(
    () => import('@/app/ai-assistant/page'),
    { preloadAfter: 3000 }
  ),
  
  // Search
  Search: () => lazyWithRetry(
    () => import('@/app/search/page')
  ),
  
  // Hadith
  Hadith: () => lazyWithRetry(
    () => import('@/app/hadith/page')
  ),
  
  // Stories
  Stories: () => lazyWithRetry(
    () => import('@/app/stories/page')
  ),
  
  // Recording
  Recording: () => lazyWithRetry(
    () => import('@/app/recording/page')
  ),
  
  // Qibla
  Qibla: () => lazyWithRetry(
    () => import('@/app/qibla/page')
  ),
  
  // Prayer Times
  PrayerTimes: () => lazyWithRetry(
    () => import('@/app/prayer-times/page')
  ),
  
  // Statistics
  Statistics: () => lazyWithRetry(
    () => import('@/app/statistics/page')
  ),
  
  // Achievements
  Achievements: () => lazyWithRetry(
    () => import('@/app/achievements/page')
  ),
  
  // Settings
  Settings: () => lazyWithRetry(
    () => import('@/app/settings/page')
  ),
  
  // Accessibility
  Accessibility: () => lazyWithRetry(
    () => import('@/app/accessibility/page')
  ),
  
  // Downloads
  Downloads: () => lazyWithRetry(
    () => import('@/app/downloads/page')
  ),
} as const;

/**
 * Component-based code splitting configuration
 */
export const LazyComponents = {
  // Dashboard Components
  PrayerTimesWidget: () => lazyWithRetry(
    () => import('@/components/dashboard/PrayerTimesWidget')
  ),
  KhatmaProgressWidget: () => lazyWithRetry(
    () => import('@/components/dashboard/KhatmaProgressWidget')
  ),
  DailyVerseWidget: () => lazyWithRetry(
    () => import('@/components/dashboard/DailyVerseWidget')
  ),
  WeatherWidget: () => lazyWithRetry(
    () => import('@/components/dashboard/WeatherWidget')
  ),
  
  // Quran Components
  SurahCard: () => lazyWithRetry(
    () => import('@/components/quran/SurahCard')
  ),
  JuzCard: () => lazyWithRetry(
    () => import('@/components/quran/JuzCard')
  ),
  BookmarkCard: () => lazyWithRetry(
    () => import('@/components/quran/BookmarkCard')
  ),
  AyahOptionsModal: () => lazyWithRetry(
    () => import('@/components/quran/AyahOptionsModal')
  ),
  
  // Tafsir Components
  TafsirViewer: () => lazyWithRetry(
    () => import('@/components/tafsir/TafsirViewer')
  ),
  TafsirComparison: () => lazyWithRetry(
    () => import('@/components/tafsir/TafsirComparison')
  ),
  
  // AI Assistant Components
  MessageBubble: () => lazyWithRetry(
    () => import('@/components/ai-assistant/MessageBubble')
  ),
  SourceCard: () => lazyWithRetry(
    () => import('@/components/ai-assistant/SourceCard')
  ),
  ChatInput: () => lazyWithRetry(
    () => import('@/components/ai-assistant/ChatInput')
  ),
  
  // Search Components
  SearchBar: () => lazyWithRetry(
    () => import('@/components/search/SearchBar')
  ),
  SearchResults: () => lazyWithRetry(
    () => import('@/components/search/SearchResults')
  ),
  SearchFilters: () => lazyWithRetry(
    () => import('@/components/search/SearchFilters')
  ),
  
  // Recording Components
  WaveformVisualizer: () => lazyWithRetry(
    () => import('@/components/recording/WaveformVisualizer')
  ),
  RecordingControls: () => lazyWithRetry(
    () => import('@/components/recording/RecordingControls')
  ),
  AnalysisResults: () => lazyWithRetry(
    () => import('@/components/recording/AnalysisResults')
  ),
  
  // Qibla Components
  CompassVisualization: () => lazyWithRetry(
    () => import('@/components/qibla/CompassVisualization')
  ),
  QiblaInfoPanel: () => lazyWithRetry(
    () => import('@/components/qibla/QiblaInfoPanel')
  ),
  
  // UI Components (Heavy)
  LazyLoadingList: () => lazyWithRetry(
    () => import('@/components/ui/LazyLoadingList')
  ),
  IslamicModal: () => lazyWithRetry(
    () => import('@/components/ui/IslamicModal')
  ),
} as const;

/**
 * Service-based code splitting
 */
export const LazyServices = {
  // AI Assistant Service
  AIAssistantService: () => import('@/lib/services/ai-assistant-service'),
  
  // Recording Service
  RecordingService: () => import('@/lib/services/recording-service'),
  
  // Search Service
  SearchService: () => import('@/lib/services/search-service'),
  
  // Download Manager
  DownloadManagerService: () => import('@/lib/services/download-manager-service'),
  
  // Voice Navigation
  VoiceNavigationService: () => import('@/lib/services/voice-navigation-service'),
  
  // Keyboard Shortcuts
  KeyboardShortcutsService: () => import('@/lib/services/keyboard-shortcuts-service'),
} as const;

/**
 * Utility-based code splitting
 */
export const LazyUtils = {
  // Animation utilities
  Animations: () => import('@/lib/utils/animations'),
  
  // Data transformations
  DataTransformations: () => import('@/lib/utils/data-transformations'),
  
  // Compression utilities
  Compression: () => import('@/lib/utils/compression'),
} as const;

/**
 * Preload critical routes
 */
export function preloadCriticalRoutes(): void {
  // Preload dashboard and Quran index (most commonly accessed)
  RouteComponents.Dashboard();
  RouteComponents.QuranIndex();
}

/**
 * Preload route based on user navigation patterns
 */
export function preloadRouteByPattern(currentRoute: string): void {
  const preloadMap: Record<string, () => void> = {
    '/': () => {
      RouteComponents.Dashboard();
      RouteComponents.QuranIndex();
    },
    '/dashboard': () => {
      RouteComponents.QuranIndex();
      RouteComponents.PrayerTimes();
    },
    '/quran': () => {
      RouteComponents.QuranMushaf();
      LazyComponents.TafsirViewer();
    },
    '/ai-assistant': () => {
      LazyComponents.MessageBubble();
      LazyComponents.SourceCard();
    },
    '/search': () => {
      LazyComponents.SearchResults();
      LazyComponents.SearchFilters();
    },
  };
  
  const preloadFunc = preloadMap[currentRoute];
  if (preloadFunc) {
    // Preload after a short delay
    setTimeout(preloadFunc, 1000);
  }
}

/**
 * Dynamic import with error boundary
 */
export async function safeDynamicImport<T>(
  importFunc: () => Promise<T>,
  fallback?: T
): Promise<T> {
  try {
    return await importFunc();
  } catch (error) {
    console.error('Failed to dynamically import module:', error);
    if (fallback) {
      return fallback;
    }
    throw error;
  }
}

/**
 * Chunk loading priority
 */
export enum ChunkPriority {
  CRITICAL = 'critical',
  HIGH = 'high',
  MEDIUM = 'medium',
  LOW = 'low',
}

/**
 * Load chunk with priority
 */
export async function loadChunkWithPriority<T>(
  importFunc: () => Promise<T>,
  priority: ChunkPriority = ChunkPriority.MEDIUM
): Promise<T> {
  // In production, this could integrate with resource hints
  // For now, we'll use a simple delay based on priority
  const delays: Record<ChunkPriority, number> = {
    [ChunkPriority.CRITICAL]: 0,
    [ChunkPriority.HIGH]: 100,
    [ChunkPriority.MEDIUM]: 500,
    [ChunkPriority.LOW]: 1000,
  };
  
  const delay = delays[priority];
  if (delay > 0) {
    await new Promise(resolve => setTimeout(resolve, delay));
  }
  
  return importFunc();
}

/**
 * Bundle size tracking
 */
interface BundleInfo {
  name: string;
  size: number;
  loadTime: number;
}

const bundleMetrics: BundleInfo[] = [];

/**
 * Track bundle loading
 */
export async function trackBundleLoad<T>(
  name: string,
  importFunc: () => Promise<T>
): Promise<T> {
  const startTime = performance.now();
  
  try {
    const module = await importFunc();
    const loadTime = performance.now() - startTime;
    
    // Estimate size (this is approximate)
    const size = new Blob([JSON.stringify(module)]).size;
    
    bundleMetrics.push({
      name,
      size,
      loadTime,
    });
    
    // Log if loading is slow
    if (loadTime > 1000) {
      console.warn(`Slow bundle load: ${name} took ${loadTime.toFixed(2)}ms`);
    }
    
    return module;
  } catch (error) {
    console.error(`Failed to load bundle: ${name}`, error);
    throw error;
  }
}

/**
 * Get bundle metrics
 */
export function getBundleMetrics(): BundleInfo[] {
  return [...bundleMetrics];
}

/**
 * Clear bundle metrics
 */
export function clearBundleMetrics(): void {
  bundleMetrics.length = 0;
}

/**
 * Analyze bundle performance
 */
export function analyzeBundlePerformance(): {
  totalSize: number;
  totalLoadTime: number;
  averageLoadTime: number;
  slowestBundles: BundleInfo[];
  largestBundles: BundleInfo[];
} {
  const totalSize = bundleMetrics.reduce((sum, b) => sum + b.size, 0);
  const totalLoadTime = bundleMetrics.reduce((sum, b) => sum + b.loadTime, 0);
  const averageLoadTime = totalLoadTime / bundleMetrics.length || 0;
  
  const slowestBundles = [...bundleMetrics]
    .sort((a, b) => b.loadTime - a.loadTime)
    .slice(0, 5);
  
  const largestBundles = [...bundleMetrics]
    .sort((a, b) => b.size - a.size)
    .slice(0, 5);
  
  return {
    totalSize,
    totalLoadTime,
    averageLoadTime,
    slowestBundles,
    largestBundles,
  };
}

/**
 * Prefetch all lazy components for a route
 */
export function prefetchRouteComponents(route: string): void {
  const componentMap: Record<string, Array<() => void>> = {
    '/dashboard': [
      LazyComponents.PrayerTimesWidget,
      LazyComponents.KhatmaProgressWidget,
      LazyComponents.DailyVerseWidget,
    ],
    '/quran': [
      LazyComponents.SurahCard,
      LazyComponents.JuzCard,
      LazyComponents.BookmarkCard,
    ],
    '/ai-assistant': [
      LazyComponents.MessageBubble,
      LazyComponents.SourceCard,
      LazyComponents.ChatInput,
    ],
    '/search': [
      LazyComponents.SearchBar,
      LazyComponents.SearchResults,
      LazyComponents.SearchFilters,
    ],
  };
  
  const components = componentMap[route];
  if (components) {
    components.forEach(component => {
      setTimeout(component, 500);
    });
  }
}

/**
 * Check if module is already loaded
 */
export function isModuleLoaded(moduleName: string): boolean {
  // This is a simplified check
  // In production, you'd check the webpack module cache
  return typeof window !== 'undefined' && 
         (window as any).__NEXT_DATA__?.props?.pageProps?.[moduleName] !== undefined;
}

/**
 * Preload module if not already loaded
 */
export async function preloadIfNeeded<T>(
  moduleName: string,
  importFunc: () => Promise<T>
): Promise<T | null> {
  if (isModuleLoaded(moduleName)) {
    return null;
  }
  
  return importFunc();
}
