/**
 * Advanced Lazy Loading Utilities
 * 
 * Provides utilities for lazy loading components, images, and data
 * with performance optimizations and error handling.
 */

import { ComponentType, lazy, LazyExoticComponent } from 'react';

/**
 * Options for lazy loading components
 */
interface LazyLoadOptions {
  /**
   * Delay before loading the component (in ms)
   * Useful for preventing flash of loading state
   */
  delay?: number;
  
  /**
   * Preload the component after a delay (in ms)
   * Useful for components that will likely be needed soon
   */
  preloadAfter?: number;
  
  /**
   * Retry loading if it fails
   */
  retry?: boolean;
  
  /**
   * Number of retry attempts
   */
  retryAttempts?: number;
  
  /**
   * Delay between retry attempts (in ms)
   */
  retryDelay?: number;
}

/**
 * Enhanced lazy loading with retry logic and preloading
 */
export function lazyWithRetry<T extends ComponentType<any>>(
  importFunc: () => Promise<{ default: T }>,
  options: LazyLoadOptions = {}
): LazyExoticComponent<T> {
  const {
    delay = 0,
    preloadAfter,
    retry = true,
    retryAttempts = 3,
    retryDelay = 1000,
  } = options;

  let retryCount = 0;

  const loadComponent = async (): Promise<{ default: T }> => {
    try {
      // Add artificial delay if specified
      if (delay > 0) {
        await new Promise(resolve => setTimeout(resolve, delay));
      }

      const component = await importFunc();
      retryCount = 0; // Reset retry count on success
      return component;
    } catch (error) {
      if (retry && retryCount < retryAttempts) {
        retryCount++;
        console.warn(
          `Failed to load component, retrying (${retryCount}/${retryAttempts})...`,
          error
        );
        
        // Wait before retrying
        await new Promise(resolve => setTimeout(resolve, retryDelay));
        return loadComponent();
      }
      
      throw error;
    }
  };

  const LazyComponent = lazy(loadComponent);

  // Preload after specified delay
  if (preloadAfter && preloadAfter > 0) {
    setTimeout(() => {
      loadComponent().catch(() => {
        // Ignore preload errors
      });
    }, preloadAfter);
  }

  return LazyComponent;
}

/**
 * Lazy load a component with a minimum loading time
 * Prevents flash of loading state for fast-loading components
 */
export function lazyWithMinLoadTime<T extends ComponentType<any>>(
  importFunc: () => Promise<{ default: T }>,
  minLoadTime: number = 300
): LazyExoticComponent<T> {
  return lazy(async () => {
    const [component] = await Promise.all([
      importFunc(),
      new Promise(resolve => setTimeout(resolve, minLoadTime)),
    ]);
    return component;
  });
}

/**
 * Preload a lazy component
 */
export function preloadComponent<T extends ComponentType<any>>(
  LazyComponent: LazyExoticComponent<T>
): void {
  // @ts-ignore - accessing internal preload method
  if (LazyComponent._payload && LazyComponent._payload._result === null) {
    // @ts-ignore
    LazyComponent._payload._result = LazyComponent._payload._fn();
  }
}

/**
 * Intersection Observer options for lazy loading
 */
interface IntersectionObserverOptions {
  /**
   * Root margin for intersection observer
   */
  rootMargin?: string;
  
  /**
   * Threshold for intersection observer
   */
  threshold?: number | number[];
  
  /**
   * Whether to unobserve after first intersection
   */
  once?: boolean;
}

/**
 * Create an intersection observer for lazy loading
 */
export function createLazyLoadObserver(
  callback: (entry: IntersectionObserverEntry) => void,
  options: IntersectionObserverOptions = {}
): IntersectionObserver {
  const {
    rootMargin = '50px',
    threshold = 0.01,
    once = true,
  } = options;

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          callback(entry);
          if (once) {
            observer.unobserve(entry.target);
          }
        }
      });
    },
    {
      rootMargin,
      threshold,
    }
  );

  return observer;
}

/**
 * Lazy load images with intersection observer
 */
export function lazyLoadImage(
  img: HTMLImageElement,
  options: IntersectionObserverOptions = {}
): () => void {
  const dataSrc = img.dataset.src;
  const dataSrcset = img.dataset.srcset;

  if (!dataSrc && !dataSrcset) {
    return () => {};
  }

  const observer = createLazyLoadObserver(
    () => {
      if (dataSrc) {
        img.src = dataSrc;
      }
      if (dataSrcset) {
        img.srcset = dataSrcset;
      }
      img.classList.add('loaded');
    },
    options
  );

  observer.observe(img);

  // Return cleanup function
  return () => {
    observer.unobserve(img);
    observer.disconnect();
  };
}

/**
 * Batch lazy load multiple images
 */
export function lazyLoadImages(
  selector: string = 'img[data-src]',
  options: IntersectionObserverOptions = {}
): () => void {
  const images = document.querySelectorAll<HTMLImageElement>(selector);
  const cleanupFunctions: Array<() => void> = [];

  images.forEach((img) => {
    const cleanup = lazyLoadImage(img, options);
    cleanupFunctions.push(cleanup);
  });

  // Return cleanup function for all images
  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

/**
 * Prefetch data for a route
 */
export async function prefetchRoute(href: string): Promise<void> {
  try {
    // Use Next.js router prefetch if available
    if (typeof window !== 'undefined' && 'next' in window) {
      const router = (window as any).next?.router;
      if (router && router.prefetch) {
        await router.prefetch(href);
        return;
      }
    }

    // Fallback to fetch
    await fetch(href, { method: 'HEAD' });
  } catch (error) {
    console.warn('Failed to prefetch route:', href, error);
  }
}

/**
 * Prefetch multiple routes
 */
export async function prefetchRoutes(hrefs: string[]): Promise<void> {
  await Promise.all(hrefs.map(href => prefetchRoute(href)));
}

/**
 * Lazy load a script
 */
export function lazyLoadScript(
  src: string,
  options: {
    async?: boolean;
    defer?: boolean;
    onLoad?: () => void;
    onError?: (error: Error) => void;
  } = {}
): Promise<void> {
  return new Promise((resolve, reject) => {
    // Check if script already exists
    const existingScript = document.querySelector(`script[src="${src}"]`);
    if (existingScript) {
      resolve();
      return;
    }

    const script = document.createElement('script');
    script.src = src;
    script.async = options.async ?? true;
    script.defer = options.defer ?? false;

    script.onload = () => {
      options.onLoad?.();
      resolve();
    };

    script.onerror = () => {
      const error = new Error(`Failed to load script: ${src}`);
      options.onError?.(error);
      reject(error);
    };

    document.body.appendChild(script);
  });
}

/**
 * Lazy load CSS
 */
export function lazyLoadCSS(
  href: string,
  options: {
    media?: string;
    onLoad?: () => void;
    onError?: (error: Error) => void;
  } = {}
): Promise<void> {
  return new Promise((resolve, reject) => {
    // Check if stylesheet already exists
    const existingLink = document.querySelector(`link[href="${href}"]`);
    if (existingLink) {
      resolve();
      return;
    }

    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = href;
    link.media = options.media || 'all';

    link.onload = () => {
      options.onLoad?.();
      resolve();
    };

    link.onerror = () => {
      const error = new Error(`Failed to load stylesheet: ${href}`);
      options.onError?.(error);
      reject(error);
    };

    document.head.appendChild(link);
  });
}

/**
 * Priority hints for resource loading
 */
export type ResourcePriority = 'high' | 'low' | 'auto';

/**
 * Preload a resource with priority hints
 */
export function preloadResource(
  href: string,
  as: 'script' | 'style' | 'image' | 'font' | 'fetch',
  options: {
    priority?: ResourcePriority;
    crossOrigin?: 'anonymous' | 'use-credentials';
    type?: string;
  } = {}
): void {
  const link = document.createElement('link');
  link.rel = 'preload';
  link.href = href;
  link.as = as;

  if (options.priority) {
    link.setAttribute('fetchpriority', options.priority);
  }

  if (options.crossOrigin) {
    link.crossOrigin = options.crossOrigin;
  }

  if (options.type) {
    link.type = options.type;
  }

  document.head.appendChild(link);
}

/**
 * Prefetch a resource for future navigation
 */
export function prefetchResource(href: string): void {
  const link = document.createElement('link');
  link.rel = 'prefetch';
  link.href = href;
  document.head.appendChild(link);
}

/**
 * DNS prefetch for external domains
 */
export function dnsPrefetch(domain: string): void {
  const link = document.createElement('link');
  link.rel = 'dns-prefetch';
  link.href = domain;
  document.head.appendChild(link);
}

/**
 * Preconnect to external domains
 */
export function preconnect(
  domain: string,
  crossOrigin?: 'anonymous' | 'use-credentials'
): void {
  const link = document.createElement('link');
  link.rel = 'preconnect';
  link.href = domain;
  
  if (crossOrigin) {
    link.crossOrigin = crossOrigin;
  }
  
  document.head.appendChild(link);
}

/**
 * Check if user prefers reduced data usage
 */
export function prefersReducedData(): boolean {
  if (typeof navigator === 'undefined') return false;
  
  // Check for Save-Data header
  const connection = (navigator as any).connection;
  if (connection && connection.saveData) {
    return true;
  }
  
  // Check for slow connection
  if (connection && connection.effectiveType) {
    return ['slow-2g', '2g'].includes(connection.effectiveType);
  }
  
  return false;
}

/**
 * Get network information
 */
export function getNetworkInfo(): {
  effectiveType?: string;
  downlink?: number;
  rtt?: number;
  saveData?: boolean;
} {
  if (typeof navigator === 'undefined') return {};
  
  const connection = (navigator as any).connection;
  if (!connection) return {};
  
  return {
    effectiveType: connection.effectiveType,
    downlink: connection.downlink,
    rtt: connection.rtt,
    saveData: connection.saveData,
  };
}

/**
 * Adaptive loading based on network conditions
 */
export function shouldLoadHighQuality(): boolean {
  if (prefersReducedData()) return false;
  
  const networkInfo = getNetworkInfo();
  
  // Load high quality on fast connections
  if (networkInfo.effectiveType === '4g' && (networkInfo.downlink ?? 0) > 1.5) {
    return true;
  }
  
  // Default to lower quality
  return false;
}
