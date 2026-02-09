import pako from 'pako';

/**
 * Storage priority levels for content
 */
export enum StoragePriority {
  CRITICAL = 0, // Essential content (Quran text, prayer times)
  HIGH = 1,     // Frequently accessed (bookmarks, recent content)
  MEDIUM = 2,   // Cached content (tafsir, hadith)
  LOW = 3,      // Optional content (images, audio)
}

/**
 * Storage item metadata
 */
export interface StorageItem {
  key: string;
  size: number;
  lastAccessed: Date;
  createdAt: Date;
  priority: StoragePriority;
  compressed: boolean;
  checksum?: string;
}

/**
 * Storage statistics
 */
export interface StorageStats {
  totalSize: number;
  availableSpace: number;
  usedSpace: number;
  itemCount: number;
  sizeByPriority: Record<StoragePriority, number>;
  lastCleanup: Date;
}

/**
 * Local storage service with smart space management for web
 */
export class LocalStorageService {
  private static readonly MAX_STORAGE_SIZE = 500 * 1024 * 1024; // 500MB
  private static readonly COMPRESSION_THRESHOLD = 10 * 1024; // 10KB
  private static readonly CLEANUP_INTERVAL = 7 * 24 * 60 * 60 * 1000; // 7 days
  private static readonly OLD_CONTENT_THRESHOLD = 30 * 24 * 60 * 60 * 1000; // 30 days

  private static readonly METADATA_PREFIX = '_metadata_';
  private static readonly DATA_PREFIX = '_data_';
  private static readonly LAST_CLEANUP_KEY = '_last_cleanup';

  /**
   * Store data with automatic compression and space management
   */
  static async store(
    key: string,
    data: any,
    priority: StoragePriority = StoragePriority.MEDIUM,
    forceCompression: boolean = false
  ): Promise<void> {
    try {
      // Serialize data
      const serialized = JSON.stringify(data);
      const bytes = new TextEncoder().encode(serialized);
      const originalSize = bytes.length;

      // Compress if needed
      let compressed = false;
      let finalData: Uint8Array = bytes;

      if (forceCompression || originalSize > this.COMPRESSION_THRESHOLD) {
        finalData = pako.gzip(bytes);
        compressed = true;
      }

      // Calculate checksum
      const checksum = await this.calculateChecksum(finalData);

      // Check if we need to free space
      await this.ensureSpace(finalData.length, priority);

      // Store data in IndexedDB
      await this.storeInIndexedDB(this.DATA_PREFIX + key, finalData);

      // Store metadata
      const metadata: StorageItem = {
        key,
        size: finalData.length,
        lastAccessed: new Date(),
        createdAt: new Date(),
        priority,
        compressed,
        checksum,
      };

      localStorage.setItem(
        this.METADATA_PREFIX + key,
        JSON.stringify(metadata)
      );
    } catch (error) {
      console.error('Failed to store data:', error);
      throw error;
    }
  }

  /**
   * Retrieve data with automatic decompression
   */
  static async retrieve<T>(key: string): Promise<T | null> {
    try {
      // Get metadata
      const metadataStr = localStorage.getItem(this.METADATA_PREFIX + key);
      if (!metadataStr) return null;

      const metadata: StorageItem = JSON.parse(metadataStr);

      // Update last accessed time
      metadata.lastAccessed = new Date();
      localStorage.setItem(
        this.METADATA_PREFIX + key,
        JSON.stringify(metadata)
      );

      // Get data from IndexedDB
      const data = await this.retrieveFromIndexedDB(this.DATA_PREFIX + key);
      if (!data) return null;

      // Verify checksum
      if (metadata.checksum) {
        const currentChecksum = await this.calculateChecksum(data);
        if (currentChecksum !== metadata.checksum) {
          console.warn('Checksum mismatch for key:', key);
          await this.remove(key);
          return null;
        }
      }

      // Decompress if needed
      let bytes = data;
      if (metadata.compressed) {
        bytes = pako.ungzip(data);
      }

      // Deserialize
      const text = new TextDecoder().decode(bytes);
      return JSON.parse(text) as T;
    } catch (error) {
      console.error('Failed to retrieve data:', error);
      return null;
    }
  }

  /**
   * Check if key exists
   */
  static has(key: string): boolean {
    return localStorage.getItem(this.METADATA_PREFIX + key) !== null;
  }

  /**
   * Remove item
   */
  static async remove(key: string): Promise<void> {
    localStorage.removeItem(this.METADATA_PREFIX + key);
    await this.removeFromIndexedDB(this.DATA_PREFIX + key);
  }

  /**
   * Get storage statistics
   */
  static async getStats(): Promise<StorageStats> {
    let totalUsed = 0;
    const sizeByPriority: Record<StoragePriority, number> = {
      [StoragePriority.CRITICAL]: 0,
      [StoragePriority.HIGH]: 0,
      [StoragePriority.MEDIUM]: 0,
      [StoragePriority.LOW]: 0,
    };

    let itemCount = 0;

    // Iterate through all metadata
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(this.METADATA_PREFIX)) {
        const metadataStr = localStorage.getItem(key);
        if (metadataStr) {
          const metadata: StorageItem = JSON.parse(metadataStr);
          totalUsed += metadata.size;
          sizeByPriority[metadata.priority] += metadata.size;
          itemCount++;
        }
      }
    }

    const lastCleanupStr = localStorage.getItem(this.LAST_CLEANUP_KEY);
    const lastCleanup = lastCleanupStr
      ? new Date(lastCleanupStr)
      : new Date(2000, 0, 1);

    return {
      totalSize: this.MAX_STORAGE_SIZE,
      availableSpace: this.MAX_STORAGE_SIZE - totalUsed,
      usedSpace: totalUsed,
      itemCount,
      sizeByPriority,
      lastCleanup,
    };
  }

  /**
   * Smart cleanup - removes old and low-priority content
   */
  static async performCleanup(force: boolean = false): Promise<void> {
    const stats = await this.getStats();
    const now = Date.now();

    if (!force && now - stats.lastCleanup.getTime() < this.CLEANUP_INTERVAL) {
      return; // Too soon for cleanup
    }

    const items: StorageItem[] = [];

    // Collect all items
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(this.METADATA_PREFIX)) {
        const metadataStr = localStorage.getItem(key);
        if (metadataStr) {
          items.push(JSON.parse(metadataStr));
        }
      }
    }

    // Sort by priority (low first) and age (old first)
    items.sort((a, b) => {
      // Critical items are never removed
      if (a.priority === StoragePriority.CRITICAL) return 1;
      if (b.priority === StoragePriority.CRITICAL) return -1;

      // Compare by priority first
      const priorityCompare = b.priority - a.priority;
      if (priorityCompare !== 0) return priorityCompare;

      // Then by age
      return (
        new Date(a.lastAccessed).getTime() -
        new Date(b.lastAccessed).getTime()
      );
    });

    // Remove old content
    let freedSpace = 0;
    const usagePercentage = (stats.usedSpace / stats.totalSize) * 100;

    for (const item of items) {
      if (item.priority === StoragePriority.CRITICAL) continue;

      const age = now - new Date(item.lastAccessed).getTime();
      const shouldRemove =
        age > this.OLD_CONTENT_THRESHOLD ||
        (usagePercentage > 80 && item.priority === StoragePriority.LOW);

      if (shouldRemove) {
        await this.remove(item.key);
        freedSpace += item.size;

        // Stop if we've freed enough space
        if (usagePercentage < 95 && freedSpace > this.MAX_STORAGE_SIZE * 0.2) {
          break;
        }
      }
    }

    localStorage.setItem(this.LAST_CLEANUP_KEY, new Date().toISOString());
    console.log(`Cleanup completed. Freed ${(freedSpace / 1024).toFixed(0)}KB`);
  }

  /**
   * Ensure enough space is available
   */
  private static async ensureSpace(
    requiredSize: number,
    priority: StoragePriority
  ): Promise<void> {
    const stats = await this.getStats();

    if (stats.availableSpace >= requiredSize) {
      return; // Enough space available
    }

    // Need to free space
    const items: StorageItem[] = [];

    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(this.METADATA_PREFIX)) {
        const metadataStr = localStorage.getItem(key);
        if (metadataStr) {
          const item: StorageItem = JSON.parse(metadataStr);
          // Only consider items with lower priority
          if (item.priority > priority) {
            items.push(item);
          }
        }
      }
    }

    // Sort by priority (low first) and age (old first)
    items.sort((a, b) => {
      const priorityCompare = b.priority - a.priority;
      if (priorityCompare !== 0) return priorityCompare;
      return (
        new Date(a.lastAccessed).getTime() -
        new Date(b.lastAccessed).getTime()
      );
    });

    let freedSpace = 0;
    for (const item of items) {
      await this.remove(item.key);
      freedSpace += item.size;

      if (freedSpace >= requiredSize) {
        break;
      }
    }

    if (freedSpace < requiredSize) {
      throw new Error('Unable to free enough space for storage');
    }
  }

  /**
   * Calculate checksum using SubtleCrypto
   */
  private static async calculateChecksum(data: Uint8Array): Promise<string> {
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
  }

  /**
   * Store data in IndexedDB
   */
  private static async storeInIndexedDB(
    key: string,
    data: Uint8Array
  ): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('SanadStorage', 1);

      request.onerror = () => reject(request.error);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains('data')) {
          db.createObjectStore('data');
        }
      };

      request.onsuccess = () => {
        const db = request.result;
        const transaction = db.transaction(['data'], 'readwrite');
        const store = transaction.objectStore('data');
        const putRequest = store.put(data, key);

        putRequest.onsuccess = () => resolve();
        putRequest.onerror = () => reject(putRequest.error);
      };
    });
  }

  /**
   * Retrieve data from IndexedDB
   */
  private static async retrieveFromIndexedDB(
    key: string
  ): Promise<Uint8Array | null> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('SanadStorage', 1);

      request.onerror = () => reject(request.error);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains('data')) {
          db.createObjectStore('data');
        }
      };

      request.onsuccess = () => {
        const db = request.result;
        const transaction = db.transaction(['data'], 'readonly');
        const store = transaction.objectStore('data');
        const getRequest = store.get(key);

        getRequest.onsuccess = () => resolve(getRequest.result || null);
        getRequest.onerror = () => reject(getRequest.error);
      };
    });
  }

  /**
   * Remove data from IndexedDB
   */
  private static async removeFromIndexedDB(key: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('SanadStorage', 1);

      request.onerror = () => reject(request.error);

      request.onsuccess = () => {
        const db = request.result;
        const transaction = db.transaction(['data'], 'readwrite');
        const store = transaction.objectStore('data');
        const deleteRequest = store.delete(key);

        deleteRequest.onsuccess = () => resolve();
        deleteRequest.onerror = () => reject(deleteRequest.error);
      };
    });
  }

  /**
   * Clear all storage
   */
  static async clearAll(): Promise<void> {
    // Clear localStorage metadata
    const keysToRemove: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(this.METADATA_PREFIX) || key?.startsWith(this.DATA_PREFIX)) {
        keysToRemove.push(key);
      }
    }
    keysToRemove.forEach((key) => localStorage.removeItem(key));

    // Clear IndexedDB
    return new Promise((resolve, reject) => {
      const request = indexedDB.deleteDatabase('SanadStorage');
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }
}
