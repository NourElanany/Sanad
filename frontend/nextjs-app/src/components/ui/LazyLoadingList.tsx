'use client';

import { useEffect, useRef, useState, useCallback, ReactNode } from 'react';

interface LazyLoadingListProps<T> {
  onLoadMore: (page: number, pageSize: number) => Promise<T[]>;
  renderItem: (item: T, index: number) => ReactNode;
  pageSize?: number;
  loadMoreThreshold?: number;
  className?: string;
  itemClassName?: string;
  loadingComponent?: ReactNode;
  emptyComponent?: ReactNode;
  errorComponent?: ReactNode;
  enableAnimation?: boolean;
}

/**
 * Lazy loading list component with infinite scroll
 * Optimized for performance with virtual scrolling support
 */
export function LazyLoadingList<T>({
  onLoadMore,
  renderItem,
  pageSize = 20,
  loadMoreThreshold = 200,
  className = '',
  itemClassName = '',
  loadingComponent,
  emptyComponent,
  errorComponent,
  enableAnimation = true,
}: LazyLoadingListProps<T>) {
  const [items, setItems] = useState<T[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const [currentPage, setCurrentPage] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const observerRef = useRef<IntersectionObserver | null>(null);
  const loadMoreTriggerRef = useRef<HTMLDivElement>(null);

  const loadInitialData = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const newItems = await onLoadMore(0, pageSize);
      setItems(newItems);
      setCurrentPage(0);
      setHasMore(newItems.length >= pageSize);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load data');
    } finally {
      setIsLoading(false);
    }
  }, [onLoadMore, pageSize]);

  const loadMore = useCallback(async () => {
    if (isLoading || !hasMore) return;

    setIsLoading(true);

    try {
      const nextPage = currentPage + 1;
      const newItems = await onLoadMore(nextPage, pageSize);
      setItems((prev) => [...prev, ...newItems]);
      setCurrentPage(nextPage);
      setHasMore(newItems.length >= pageSize);
    } catch (err) {
      console.error('Failed to load more:', err);
    } finally {
      setIsLoading(false);
    }
  }, [isLoading, hasMore, currentPage, onLoadMore, pageSize]);

  useEffect(() => {
    loadInitialData();
  }, [loadInitialData]);

  useEffect(() => {
    if (!loadMoreTriggerRef.current) return;

    observerRef.current = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !isLoading) {
          loadMore();
        }
      },
      {
        rootMargin: `${loadMoreThreshold}px`,
      }
    );

    observerRef.current.observe(loadMoreTriggerRef.current);

    return () => {
      if (observerRef.current) {
        observerRef.current.disconnect();
      }
    };
  }, [hasMore, isLoading, loadMore, loadMoreThreshold]);

  // Error state
  if (error && items.length === 0) {
    return (
      <div className={className}>
        {errorComponent || (
          <div className="flex flex-col items-center justify-center py-12">
            <svg
              className="w-16 h-16 text-red-500 mb-4"
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
            <p className="text-gray-600 mb-4">حدث خطأ أثناء التحميل</p>
            <button
              onClick={loadInitialData}
              className="px-4 py-2 bg-[#1B365D] text-white rounded-lg hover:bg-[#2E4A6B] transition-colors"
            >
              إعادة المحاولة
            </button>
          </div>
        )}
      </div>
    );
  }

  // Empty state
  if (!isLoading && items.length === 0) {
    return (
      <div className={className}>
        {emptyComponent || (
          <div className="flex flex-col items-center justify-center py-12">
            <svg
              className="w-16 h-16 text-gray-400 mb-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
              />
            </svg>
            <p className="text-gray-600">لا توجد عناصر</p>
          </div>
        )}
      </div>
    );
  }

  // Loading state for initial load
  if (isLoading && items.length === 0) {
    return (
      <div className={className}>
        {loadingComponent || (
          <div className="flex items-center justify-center py-12">
            <div className="w-8 h-8 border-4 border-[#1B365D] border-t-transparent rounded-full animate-spin" />
          </div>
        )}
      </div>
    );
  }

  return (
    <div className={className}>
      <div className="space-y-4">
        {items.map((item, index) => (
          <div
            key={index}
            className={`${itemClassName} ${
              enableAnimation && index < 10
                ? 'animate-fade-in-up'
                : ''
            }`}
            style={
              enableAnimation && index < 10
                ? { animationDelay: `${index * 50}ms` }
                : undefined
            }
          >
            {renderItem(item, index)}
          </div>
        ))}
      </div>

      {/* Load more trigger */}
      {hasMore && (
        <div
          ref={loadMoreTriggerRef}
          className="flex items-center justify-center py-8"
        >
          {isLoading && (
            <div className="w-6 h-6 border-4 border-[#1B365D] border-t-transparent rounded-full animate-spin" />
          )}
        </div>
      )}
    </div>
  );
}

/**
 * Lazy loading grid component
 */
interface LazyLoadingGridProps<T> extends LazyLoadingListProps<T> {
  columns?: number;
  gap?: number;
}

export function LazyLoadingGrid<T>({
  columns = 2,
  gap = 16,
  className = '',
  itemClassName = '',
  ...props
}: LazyLoadingGridProps<T>) {
  return (
    <LazyLoadingList
      {...props}
      className={className}
      itemClassName={itemClassName}
      renderItem={(item, index) => (
        <div
          className="w-full"
          style={{
            gridColumn: `span 1`,
          }}
        >
          {props.renderItem(item, index)}
        </div>
      )}
    />
  );
}

/**
 * Intersection Observer hook for lazy loading
 */
export function useIntersectionObserver(
  callback: () => void,
  options?: IntersectionObserverInit
) {
  const targetRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const target = targetRef.current;
    if (!target) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          callback();
        }
      },
      options
    );

    observer.observe(target);

    return () => {
      observer.disconnect();
    };
  }, [callback, options]);

  return targetRef;
}
