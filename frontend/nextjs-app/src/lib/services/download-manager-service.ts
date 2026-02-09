import { LocalStorageService, StoragePriority } from './local-storage-service';

/**
 * Download status
 */
export enum DownloadStatus {
  QUEUED = 'queued',
  DOWNLOADING = 'downloading',
  PAUSED = 'paused',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
}

/**
 * Download item
 */
export interface DownloadItem {
  id: string;
  key: string;
  title: string;
  description?: string;
  priority: StoragePriority;
  estimatedSize: number;
  downloader: () => Promise<Uint8Array>;
  status: DownloadStatus;
  downloadedBytes: number;
  error?: string;
  startedAt?: Date;
  completedAt?: Date;
}

/**
 * Download manager configuration
 */
export interface DownloadManagerConfig {
  maxConcurrentDownloads: number;
  autoRetry: boolean;
  maxRetries: number;
  retryDelay: number;
  wifiOnly: boolean;
}

/**
 * Download progress callback
 */
export type DownloadProgressCallback = (item: DownloadItem) => void;

/**
 * Download manager service for web
 */
export class DownloadManagerService {
  private downloads: Map<string, DownloadItem> = new Map();
  private queue: string[] = [];
  private activeDownloads: Set<string> = new Set();
  private retryCount: Map<string, number> = new Map();
  private listeners: Set<DownloadProgressCallback> = new Set();

  private config: DownloadManagerConfig = {
    maxConcurrentDownloads: 3,
    autoRetry: true,
    maxRetries: 3,
    retryDelay: 5000,
    wifiOnly: false,
  };

  constructor(config?: Partial<DownloadManagerConfig>) {
    if (config) {
      this.config = { ...this.config, ...config };
    }
  }

  /**
   * Subscribe to download updates
   */
  subscribe(callback: DownloadProgressCallback): () => void {
    this.listeners.add(callback);
    return () => this.listeners.delete(callback);
  }

  /**
   * Queue a download
   */
  async queueDownload(params: {
    key: string;
    title: string;
    description?: string;
    priority: StoragePriority;
    estimatedSize: number;
    downloader: () => Promise<Uint8Array>;
  }): Promise<string> {
    const id = Date.now().toString() + Math.random().toString(36).substr(2, 9);

    const item: DownloadItem = {
      id,
      key: params.key,
      title: params.title,
      description: params.description,
      priority: params.priority,
      estimatedSize: params.estimatedSize,
      downloader: params.downloader,
      status: DownloadStatus.QUEUED,
      downloadedBytes: 0,
    };

    this.downloads.set(id, item);
    this.queue.push(id);

    this.notifyListeners();
    this.processQueue();

    return id;
  }

  /**
   * Start/resume a download
   */
  async startDownload(id: string): Promise<void> {
    const item = this.downloads.get(id);
    if (!item) return;

    if (item.status === DownloadStatus.PAUSED) {
      item.status = DownloadStatus.QUEUED;
      if (!this.queue.includes(id)) {
        this.queue.push(id);
      }
      this.notifyListeners();
      this.processQueue();
    }
  }

  /**
   * Pause a download
   */
  async pauseDownload(id: string): Promise<void> {
    const item = this.downloads.get(id);
    if (!item) return;

    if (
      item.status === DownloadStatus.DOWNLOADING ||
      item.status === DownloadStatus.QUEUED
    ) {
      item.status = DownloadStatus.PAUSED;
      this.queue = this.queue.filter((qid) => qid !== id);
      this.activeDownloads.delete(id);
      this.notifyListeners();
    }
  }

  /**
   * Cancel a download
   */
  async cancelDownload(id: string): Promise<void> {
    const item = this.downloads.get(id);
    if (!item) return;

    item.status = DownloadStatus.CANCELLED;
    this.queue = this.queue.filter((qid) => qid !== id);
    this.activeDownloads.delete(id);
    this.downloads.delete(id);
    this.retryCount.delete(id);

    this.notifyListeners();
  }

  /**
   * Retry a failed download
   */
  async retryDownload(id: string): Promise<void> {
    const item = this.downloads.get(id);
    if (!item || item.status !== DownloadStatus.FAILED) return;

    item.status = DownloadStatus.QUEUED;
    item.error = undefined;
    item.downloadedBytes = 0;
    this.retryCount.set(id, 0);

    if (!this.queue.includes(id)) {
      this.queue.push(id);
    }

    this.notifyListeners();
    this.processQueue();
  }

  /**
   * Clear completed downloads
   */
  async clearCompleted(): Promise<void> {
    const completedIds: string[] = [];

    this.downloads.forEach((item, id) => {
      if (item.status === DownloadStatus.COMPLETED) {
        completedIds.push(id);
      }
    });

    completedIds.forEach((id) => this.downloads.delete(id));
    this.notifyListeners();
  }

  /**
   * Get all downloads
   */
  getDownloads(): DownloadItem[] {
    return Array.from(this.downloads.values());
  }

  /**
   * Get download by ID
   */
  getDownload(id: string): DownloadItem | undefined {
    return this.downloads.get(id);
  }

  /**
   * Get active downloads
   */
  getActiveDownloads(): DownloadItem[] {
    return this.getDownloads().filter(
      (d) =>
        d.status === DownloadStatus.DOWNLOADING ||
        d.status === DownloadStatus.QUEUED
    );
  }

  /**
   * Get completed downloads
   */
  getCompletedDownloads(): DownloadItem[] {
    return this.getDownloads().filter(
      (d) => d.status === DownloadStatus.COMPLETED
    );
  }

  /**
   * Get failed downloads
   */
  getFailedDownloads(): DownloadItem[] {
    return this.getDownloads().filter((d) => d.status === DownloadStatus.FAILED);
  }

  /**
   * Get total download size
   */
  getTotalSize(): number {
    return this.getDownloads().reduce(
      (sum, item) => sum + item.estimatedSize,
      0
    );
  }

  /**
   * Get downloaded size
   */
  getDownloadedSize(): number {
    return this.getDownloads().reduce(
      (sum, item) => sum + item.downloadedBytes,
      0
    );
  }

  /**
   * Get overall progress
   */
  getOverallProgress(): number {
    const total = this.getTotalSize();
    if (total === 0) return 0;
    return Math.min(1, this.getDownloadedSize() / total);
  }

  /**
   * Process download queue
   */
  private async processQueue(): Promise<void> {
    // Check if we can start more downloads
    while (
      this.activeDownloads.size < this.config.maxConcurrentDownloads &&
      this.queue.length > 0
    ) {
      // Sort queue by priority
      this.queue.sort((a, b) => {
        const itemA = this.downloads.get(a);
        const itemB = this.downloads.get(b);
        if (!itemA || !itemB) return 0;
        return itemA.priority - itemB.priority;
      });

      const id = this.queue.shift();
      if (!id) continue;

      const item = this.downloads.get(id);
      if (!item || item.status !== DownloadStatus.QUEUED) {
        continue;
      }

      this.activeDownloads.add(id);
      this.downloadItem(id);
    }
  }

  /**
   * Download an item
   */
  private async downloadItem(id: string): Promise<void> {
    const item = this.downloads.get(id);
    if (!item) return;

    try {
      item.status = DownloadStatus.DOWNLOADING;
      item.startedAt = new Date();
      this.notifyListeners();

      // Download data
      const data = await item.downloader();

      // Store in local storage
      await LocalStorageService.store(
        item.key,
        Array.from(data),
        item.priority
      );

      item.status = DownloadStatus.COMPLETED;
      item.completedAt = new Date();
      item.downloadedBytes = item.estimatedSize;
      this.retryCount.delete(id);

      console.log('Download completed:', item.title);
    } catch (error) {
      console.error('Download failed:', item.title, error);

      const retries = this.retryCount.get(id) || 0;

      if (this.config.autoRetry && retries < this.config.maxRetries) {
        this.retryCount.set(id, retries + 1);
        item.status = DownloadStatus.QUEUED;

        // Add back to queue after delay
        setTimeout(() => {
          if (!this.queue.includes(id)) {
            this.queue.push(id);
          }
          this.processQueue();
        }, this.config.retryDelay);

        console.log(
          `Retrying download: ${item.title} (${retries + 1}/${this.config.maxRetries})`
        );
      } else {
        item.status = DownloadStatus.FAILED;
        item.error = error instanceof Error ? error.message : String(error);
      }
    } finally {
      this.activeDownloads.delete(id);
      this.notifyListeners();
      this.processQueue();
    }
  }

  /**
   * Notify listeners
   */
  private notifyListeners(): void {
    this.downloads.forEach((item) => {
      this.listeners.forEach((callback) => callback(item));
    });
  }
}

// Singleton instance
let downloadManagerInstance: DownloadManagerService | null = null;

export function getDownloadManager(): DownloadManagerService {
  if (!downloadManagerInstance) {
    downloadManagerInstance = new DownloadManagerService();
  }
  return downloadManagerInstance;
}
