'use client';

import Image from 'next/image';
import { useState } from 'react';

interface OptimizedImageProps {
  src: string;
  alt: string;
  width?: number;
  height?: number;
  fill?: boolean;
  priority?: boolean;
  quality?: number;
  className?: string;
  objectFit?: 'contain' | 'cover' | 'fill' | 'none' | 'scale-down';
  placeholder?: 'blur' | 'empty';
  blurDataURL?: string;
  onLoad?: () => void;
  onError?: () => void;
}

/**
 * Optimized image component with lazy loading and blur placeholder
 * Uses Next.js Image component for automatic optimization
 */
export function OptimizedImage({
  src,
  alt,
  width,
  height,
  fill = false,
  priority = false,
  quality = 85,
  className = '',
  objectFit = 'cover',
  placeholder = 'blur',
  blurDataURL,
  onLoad,
  onError,
}: OptimizedImageProps) {
  const [isLoading, setIsLoading] = useState(true);
  const [hasError, setHasError] = useState(false);

  const handleLoad = () => {
    setIsLoading(false);
    onLoad?.();
  };

  const handleError = () => {
    setIsLoading(false);
    setHasError(true);
    onError?.();
  };

  if (hasError) {
    return (
      <div
        className={`flex items-center justify-center bg-gray-200 ${className}`}
        style={{ width, height }}
      >
        <svg
          className="w-12 h-12 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
          />
        </svg>
      </div>
    );
  }

  const imageProps = {
    src,
    alt,
    quality,
    priority,
    className: `${className} ${isLoading ? 'blur-sm' : 'blur-0'} transition-all duration-300`,
    onLoad: handleLoad,
    onError: handleError,
    ...(fill
      ? { fill: true, style: { objectFit } }
      : { width, height, style: { objectFit } }),
    ...(placeholder === 'blur' && blurDataURL
      ? { placeholder: 'blur' as const, blurDataURL }
      : {}),
  };

  return (
    <>
      <Image {...imageProps} />
      {isLoading && (
        <div className="absolute inset-0 flex items-center justify-center bg-gray-100">
          <div className="w-8 h-8 border-4 border-[#1B365D] border-t-transparent rounded-full animate-spin" />
        </div>
      )}
    </>
  );
}

/**
 * Optimized Quran page image component
 */
export function OptimizedQuranPageImage({
  pageNumber,
  imageUrl,
  onTap,
}: {
  pageNumber: number;
  imageUrl: string;
  onTap?: () => void;
}) {
  const [isLoading, setIsLoading] = useState(true);
  const [hasError, setHasError] = useState(false);

  return (
    <div
      className="relative w-full h-full cursor-pointer"
      onClick={onTap}
    >
      {hasError ? (
        <div className="flex flex-col items-center justify-center h-full bg-[#FEFEFE]">
          <svg
            className="w-12 h-12 text-red-500 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p className="text-gray-600 font-['Tajawal']">فشل تحميل الصفحة</p>
        </div>
      ) : (
        <>
          <Image
            src={imageUrl}
            alt={`صفحة ${pageNumber}`}
            fill
            quality={90}
            priority={pageNumber <= 2}
            className={`object-contain transition-opacity duration-300 ${
              isLoading ? 'opacity-0' : 'opacity-100'
            }`}
            onLoad={() => setIsLoading(false)}
            onError={() => {
              setIsLoading(false);
              setHasError(true);
            }}
          />
          {isLoading && (
            <div className="absolute inset-0 flex flex-col items-center justify-center bg-[#FEFEFE]">
              <div className="w-8 h-8 border-4 border-[#1B365D] border-t-transparent rounded-full animate-spin mb-4" />
              <p className="text-gray-600 font-['Tajawal'] text-sm">
                جاري تحميل الصفحة...
              </p>
            </div>
          )}
        </>
      )}
    </div>
  );
}

/**
 * Generate blur data URL for placeholder
 */
export function generateBlurDataURL(width: number = 10, height: number = 10): string {
  const canvas =
    typeof document !== 'undefined' ? document.createElement('canvas') : null;
  if (!canvas) return '';

  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  if (!ctx) return '';

  // Create gradient blur effect
  const gradient = ctx.createLinearGradient(0, 0, width, height);
  gradient.addColorStop(0, '#E0E0E0');
  gradient.addColorStop(1, '#F5F5F5');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);

  return canvas.toDataURL();
}
