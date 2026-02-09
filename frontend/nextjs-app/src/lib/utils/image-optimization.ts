/**
 * Advanced Image Optimization Utilities
 * 
 * Provides utilities for optimizing images with:
 * - Responsive images
 * - Format selection (WebP, AVIF)
 * - Lazy loading
 * - Blur placeholders
 * - Adaptive quality based on network
 */

import { getNetworkInfo, shouldLoadHighQuality } from './lazy-loading';

/**
 * Image format types
 */
export type ImageFormat = 'jpeg' | 'png' | 'webp' | 'avif' | 'gif';

/**
 * Image quality presets
 */
export const ImageQuality = {
  LOW: 50,
  MEDIUM: 75,
  HIGH: 90,
  MAX: 100,
} as const;

/**
 * Responsive image breakpoints
 */
export const ImageBreakpoints = {
  MOBILE: 640,
  TABLET: 768,
  DESKTOP: 1024,
  WIDE: 1280,
  ULTRA_WIDE: 1920,
} as const;

/**
 * Options for image optimization
 */
interface ImageOptimizationOptions {
  /**
   * Image width
   */
  width?: number;
  
  /**
   * Image height
   */
  height?: number;
  
  /**
   * Image quality (1-100)
   */
  quality?: number;
  
  /**
   * Image format
   */
  format?: ImageFormat;
  
  /**
   * Whether to use adaptive quality based on network
   */
  adaptiveQuality?: boolean;
  
  /**
   * Whether to generate blur placeholder
   */
  blurPlaceholder?: boolean;
  
  /**
   * Blur placeholder size
   */
  blurSize?: number;
  
  /**
   * Whether to use responsive images
   */
  responsive?: boolean;
  
  /**
   * Custom breakpoints for responsive images
   */
  breakpoints?: number[];
}

/**
 * Get optimal image format based on browser support
 */
export function getOptimalImageFormat(): ImageFormat {
  if (typeof window === 'undefined') return 'webp';
  
  // Check AVIF support
  const avifSupport = document.createElement('canvas')
    .toDataURL('image/avif')
    .indexOf('data:image/avif') === 0;
  
  if (avifSupport) return 'avif';
  
  // Check WebP support
  const webpSupport = document.createElement('canvas')
    .toDataURL('image/webp')
    .indexOf('data:image/webp') === 0;
  
  if (webpSupport) return 'webp';
  
  return 'jpeg';
}

/**
 * Get adaptive image quality based on network conditions
 */
export function getAdaptiveQuality(baseQuality: number = ImageQuality.HIGH): number {
  if (!shouldLoadHighQuality()) {
    // Reduce quality on slow connections
    return Math.max(ImageQuality.LOW, baseQuality - 25);
  }
  
  const networkInfo = getNetworkInfo();
  
  // Adjust quality based on connection type
  switch (networkInfo.effectiveType) {
    case '4g':
      return baseQuality;
    case '3g':
      return Math.max(ImageQuality.MEDIUM, baseQuality - 15);
    case '2g':
    case 'slow-2g':
      return ImageQuality.LOW;
    default:
      return ImageQuality.MEDIUM;
  }
}

/**
 * Generate srcset for responsive images
 */
export function generateSrcSet(
  baseUrl: string,
  options: ImageOptimizationOptions = {}
): string {
  const {
    breakpoints = Object.values(ImageBreakpoints),
    quality = ImageQuality.HIGH,
    format = getOptimalImageFormat(),
    adaptiveQuality = true,
  } = options;
  
  const actualQuality = adaptiveQuality ? getAdaptiveQuality(quality) : quality;
  
  return breakpoints
    .map((width) => {
      const url = buildImageUrl(baseUrl, {
        width,
        quality: actualQuality,
        format,
      });
      return `${url} ${width}w`;
    })
    .join(', ');
}

/**
 * Generate sizes attribute for responsive images
 */
export function generateSizes(
  breakpoints: Array<{ breakpoint: number; size: string }> = []
): string {
  if (breakpoints.length === 0) {
    return '100vw';
  }
  
  return breakpoints
    .map(({ breakpoint, size }) => `(max-width: ${breakpoint}px) ${size}`)
    .join(', ');
}

/**
 * Build optimized image URL
 */
export function buildImageUrl(
  baseUrl: string,
  options: ImageOptimizationOptions = {}
): string {
  const {
    width,
    height,
    quality = ImageQuality.HIGH,
    format = getOptimalImageFormat(),
    adaptiveQuality = true,
  } = options;
  
  const actualQuality = adaptiveQuality ? getAdaptiveQuality(quality) : quality;
  
  const params = new URLSearchParams();
  
  if (width) params.set('w', width.toString());
  if (height) params.set('h', height.toString());
  params.set('q', actualQuality.toString());
  params.set('fm', format);
  
  return `${baseUrl}?${params.toString()}`;
}

/**
 * Generate blur placeholder data URL
 */
export async function generateBlurPlaceholder(
  imageUrl: string,
  size: number = 10
): Promise<string> {
  try {
    // In a real implementation, this would generate a tiny blurred version
    // For now, return a simple gray placeholder
    const canvas = document.createElement('canvas');
    canvas.width = size;
    canvas.height = size;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return '';
    
    // Create gradient placeholder
    const gradient = ctx.createLinearGradient(0, 0, size, size);
    gradient.addColorStop(0, '#f0f0f0');
    gradient.addColorStop(1, '#e0e0e0');
    
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, size, size);
    
    return canvas.toDataURL('image/jpeg', 0.1);
  } catch (error) {
    console.error('Failed to generate blur placeholder:', error);
    return '';
  }
}

/**
 * Preload critical images
 */
export function preloadImage(
  src: string,
  options: {
    as?: 'image';
    fetchPriority?: 'high' | 'low' | 'auto';
    crossOrigin?: 'anonymous' | 'use-credentials';
  } = {}
): void {
  const link = document.createElement('link');
  link.rel = 'preload';
  link.as = options.as || 'image';
  link.href = src;
  
  if (options.fetchPriority) {
    link.setAttribute('fetchpriority', options.fetchPriority);
  }
  
  if (options.crossOrigin) {
    link.crossOrigin = options.crossOrigin;
  }
  
  document.head.appendChild(link);
}

/**
 * Preload multiple images
 */
export function preloadImages(
  srcs: string[],
  options?: Parameters<typeof preloadImage>[1]
): void {
  srcs.forEach(src => preloadImage(src, options));
}

/**
 * Calculate aspect ratio from dimensions
 */
export function calculateAspectRatio(width: number, height: number): string {
  const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));
  const divisor = gcd(width, height);
  return `${width / divisor}/${height / divisor}`;
}

/**
 * Get image dimensions from URL
 */
export async function getImageDimensions(
  url: string
): Promise<{ width: number; height: number }> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    
    img.onload = () => {
      resolve({
        width: img.naturalWidth,
        height: img.naturalHeight,
      });
    };
    
    img.onerror = () => {
      reject(new Error(`Failed to load image: ${url}`));
    };
    
    img.src = url;
  });
}

/**
 * Compress image client-side
 */
export async function compressImage(
  file: File,
  options: {
    maxWidth?: number;
    maxHeight?: number;
    quality?: number;
    format?: ImageFormat;
  } = {}
): Promise<Blob> {
  const {
    maxWidth = 1920,
    maxHeight = 1080,
    quality = ImageQuality.HIGH,
    format = 'jpeg',
  } = options;
  
  return new Promise((resolve, reject) => {
    const img = new Image();
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    
    if (!ctx) {
      reject(new Error('Failed to get canvas context'));
      return;
    }
    
    img.onload = () => {
      // Calculate new dimensions
      let { width, height } = img;
      
      if (width > maxWidth) {
        height = (height * maxWidth) / width;
        width = maxWidth;
      }
      
      if (height > maxHeight) {
        width = (width * maxHeight) / height;
        height = maxHeight;
      }
      
      // Set canvas dimensions
      canvas.width = width;
      canvas.height = height;
      
      // Draw image
      ctx.drawImage(img, 0, 0, width, height);
      
      // Convert to blob
      canvas.toBlob(
        (blob) => {
          if (blob) {
            resolve(blob);
          } else {
            reject(new Error('Failed to compress image'));
          }
        },
        `image/${format}`,
        quality / 100
      );
    };
    
    img.onerror = () => {
      reject(new Error('Failed to load image'));
    };
    
    img.src = URL.createObjectURL(file);
  });
}

/**
 * Convert image to WebP format
 */
export async function convertToWebP(
  file: File,
  quality: number = ImageQuality.HIGH
): Promise<Blob> {
  return compressImage(file, { format: 'webp', quality });
}

/**
 * Optimize image for upload
 */
export async function optimizeImageForUpload(
  file: File,
  options: {
    maxSize?: number; // in bytes
    maxWidth?: number;
    maxHeight?: number;
    quality?: number;
  } = {}
): Promise<Blob> {
  const {
    maxSize = 1024 * 1024, // 1MB
    maxWidth = 1920,
    maxHeight = 1080,
    quality = ImageQuality.HIGH,
  } = options;
  
  let compressed = await compressImage(file, {
    maxWidth,
    maxHeight,
    quality,
    format: 'webp',
  });
  
  // If still too large, reduce quality
  let currentQuality = quality;
  while (compressed.size > maxSize && currentQuality > ImageQuality.LOW) {
    currentQuality -= 10;
    compressed = await compressImage(file, {
      maxWidth,
      maxHeight,
      quality: currentQuality,
      format: 'webp',
    });
  }
  
  return compressed;
}

/**
 * Create responsive image configuration
 */
export interface ResponsiveImageConfig {
  src: string;
  srcSet: string;
  sizes: string;
  width: number;
  height: number;
  blurDataURL?: string;
}

/**
 * Generate responsive image configuration
 */
export async function generateResponsiveImageConfig(
  imageUrl: string,
  options: ImageOptimizationOptions = {}
): Promise<ResponsiveImageConfig> {
  const {
    width = 1200,
    height = 800,
    quality = ImageQuality.HIGH,
    blurPlaceholder = true,
    blurSize = 10,
  } = options;
  
  const srcSet = generateSrcSet(imageUrl, options);
  const sizes = generateSizes([
    { breakpoint: ImageBreakpoints.MOBILE, size: '100vw' },
    { breakpoint: ImageBreakpoints.TABLET, size: '90vw' },
    { breakpoint: ImageBreakpoints.DESKTOP, size: '80vw' },
  ]);
  
  const config: ResponsiveImageConfig = {
    src: buildImageUrl(imageUrl, { width, height, quality }),
    srcSet,
    sizes,
    width,
    height,
  };
  
  if (blurPlaceholder) {
    config.blurDataURL = await generateBlurPlaceholder(imageUrl, blurSize);
  }
  
  return config;
}

/**
 * Image loading strategies
 */
export const ImageLoadingStrategy = {
  /**
   * Load image immediately (for above-the-fold content)
   */
  EAGER: 'eager',
  
  /**
   * Load image when it enters viewport (default)
   */
  LAZY: 'lazy',
  
  /**
   * Preload image before it's needed
   */
  PRELOAD: 'preload',
  
  /**
   * Load image based on network conditions
   */
  ADAPTIVE: 'adaptive',
} as const;

/**
 * Determine loading strategy based on position and network
 */
export function determineLoadingStrategy(
  isAboveFold: boolean,
  isCritical: boolean = false
): string {
  if (isCritical || isAboveFold) {
    return ImageLoadingStrategy.EAGER;
  }
  
  if (shouldLoadHighQuality()) {
    return ImageLoadingStrategy.PRELOAD;
  }
  
  return ImageLoadingStrategy.LAZY;
}
