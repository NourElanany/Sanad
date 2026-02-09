// Migration service for Next.js web app
// Handles data migrations between app versions

interface MigrationStatus {
  currentMigrationVersion: number;
  targetMigrationVersion: number;
  lastAppVersion: string | null;
  availableBackups: string[];
  needsMigration: boolean;
}

interface BackupData {
  [key: string]: any;
}

class MigrationService {
  private static instance: MigrationService;
  private static readonly MIGRATION_VERSION_KEY = 'migration_version';
  private static readonly LAST_APP_VERSION_KEY = 'last_app_version';
  private static readonly CURRENT_MIGRATION_VERSION = 1;

  private constructor() {}

  static getInstance(): MigrationService {
    if (!MigrationService.instance) {
      MigrationService.instance = new MigrationService();
    }
    return MigrationService.instance;
  }

  /**
   * Initialize and run migrations if needed
   */
  async initialize(currentAppVersion: string): Promise<void> {
    if (typeof window === 'undefined') return;

    try {
      const lastMigrationVersion = this.getLastMigrationVersion();
      const lastAppVersion = this.getLastAppVersion();

      console.log(`Migration check: last=${lastMigrationVersion}, current=${MigrationService.CURRENT_MIGRATION_VERSION}`);
      console.log(`App version: last=${lastAppVersion}, current=${currentAppVersion}`);

      if (lastMigrationVersion < MigrationService.CURRENT_MIGRATION_VERSION) {
        await this.runMigrations(lastMigrationVersion, MigrationService.CURRENT_MIGRATION_VERSION);
        this.setMigrationVersion(MigrationService.CURRENT_MIGRATION_VERSION);
      }

      this.setLastAppVersion(currentAppVersion);

      console.log('Migration complete');
    } catch (error) {
      console.error('Migration error:', error);
      throw error;
    }
  }

  /**
   * Run all pending migrations
   */
  private async runMigrations(fromVersion: number, toVersion: number): Promise<void> {
    console.log(`Running migrations from v${fromVersion} to v${toVersion}`);

    for (let version = fromVersion + 1; version <= toVersion; version++) {
      console.log(`Applying migration v${version}`);
      await this.applyMigration(version);
    }
  }

  /**
   * Apply a specific migration
   */
  private async applyMigration(version: number): Promise<void> {
    switch (version) {
      case 1:
        await this.migration_v1_initial();
        break;
      // Add more migrations as needed
      default:
        console.log(`Unknown migration version: ${version}`);
    }
  }

  /**
   * Migration v1: Initial data structure setup
   */
  private async migration_v1_initial(): Promise<void> {
    console.log('Running migration v1: Initial setup');

    try {
      // Initialize IndexedDB structure
      await this.initializeIndexedDB();

      // Migrate localStorage data to IndexedDB if needed
      await this.migrateLocalStorageToIndexedDB();

      console.log('Migration v1 complete');
    } catch (error) {
      console.error('Migration v1 error:', error);
      throw error;
    }
  }

  /**
   * Initialize IndexedDB structure
   */
  private async initializeIndexedDB(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('SanadDB', 1);

      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve();

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;

        // Create object stores if they don't exist
        if (!db.objectStoreNames.contains('user_preferences')) {
          db.createObjectStore('user_preferences', { keyPath: 'id' });
        }

        if (!db.objectStoreNames.contains('quran_bookmarks')) {
          db.createObjectStore('quran_bookmarks', { keyPath: 'id', autoIncrement: true });
        }

        if (!db.objectStoreNames.contains('reading_progress')) {
          db.createObjectStore('reading_progress', { keyPath: 'id' });
        }

        if (!db.objectStoreNames.contains('offline_content')) {
          db.createObjectStore('offline_content', { keyPath: 'id' });
        }
      };
    });
  }

  /**
   * Migrate data from localStorage to IndexedDB
   */
  private async migrateLocalStorageToIndexedDB(): Promise<void> {
    console.log('Migrating localStorage to IndexedDB');

    try {
      // Migrate bookmarks
      const bookmarksJson = localStorage.getItem('bookmarks');
      if (bookmarksJson) {
        const bookmarks = JSON.parse(bookmarksJson);
        await this.saveToIndexedDB('quran_bookmarks', bookmarks);
        localStorage.removeItem('bookmarks');
      }

      // Migrate preferences
      const preferencesJson = localStorage.getItem('preferences');
      if (preferencesJson) {
        const preferences = JSON.parse(preferencesJson);
        await this.saveToIndexedDB('user_preferences', { id: 'main', ...preferences });
        localStorage.removeItem('preferences');
      }

      console.log('localStorage migration complete');
    } catch (error) {
      console.error('localStorage migration error:', error);
    }
  }

  /**
   * Save data to IndexedDB
   */
  private async saveToIndexedDB(storeName: string, data: any): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('SanadDB', 1);

      request.onerror = () => reject(request.error);

      request.onsuccess = () => {
        const db = request.result;
        const transaction = db.transaction([storeName], 'readwrite');
        const store = transaction.objectStore(storeName);

        if (Array.isArray(data)) {
          data.forEach(item => store.add(item));
        } else {
          store.put(data);
        }

        transaction.oncomplete = () => resolve();
        transaction.onerror = () => reject(transaction.error);
      };
    });
  }

  /**
   * Backup data before migration
   */
  async backupData(): Promise<void> {
    console.log('Creating data backup');

    try {
      const timestamp = new Date().toISOString();
      const backup: BackupData = {};

      // Backup localStorage
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key && !key.startsWith('backup_')) {
          backup[key] = localStorage.getItem(key);
        }
      }

      // Save backup
      localStorage.setItem(`backup_${timestamp}`, JSON.stringify(backup));

      // Keep only last 3 backups
      this.cleanupOldBackups();

      console.log(`Data backup complete: ${timestamp}`);
    } catch (error) {
      console.error('Backup error:', error);
    }
  }

  /**
   * Restore data from backup
   */
  async restoreFromBackup(backupTimestamp: string): Promise<boolean> {
    console.log(`Restoring from backup: ${backupTimestamp}`);

    try {
      const backupJson = localStorage.getItem(`backup_${backupTimestamp}`);

      if (!backupJson) {
        console.log(`Backup not found: ${backupTimestamp}`);
        return false;
      }

      const backup: BackupData = JSON.parse(backupJson);

      // Restore all data
      Object.entries(backup).forEach(([key, value]) => {
        if (value !== null) {
          localStorage.setItem(key, value as string);
        }
      });

      console.log('Data restore complete');
      return true;
    } catch (error) {
      console.error('Restore error:', error);
      return false;
    }
  }

  /**
   * Get list of available backups
   */
  getAvailableBackups(): string[] {
    const backups: string[] = [];

    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && key.startsWith('backup_')) {
        backups.push(key.replace('backup_', ''));
      }
    }

    return backups.sort((a, b) => b.localeCompare(a)); // Most recent first
  }

  /**
   * Clean up old backups (keep only last 3)
   */
  private cleanupOldBackups(): void {
    const backups = this.getAvailableBackups();

    if (backups.length > 3) {
      backups.slice(3).forEach(timestamp => {
        localStorage.removeItem(`backup_${timestamp}`);
      });
    }
  }

  /**
   * Clear all app data
   */
  async clearAllData(): Promise<void> {
    console.log('Clearing all app data');

    try {
      // Clear localStorage
      localStorage.clear();

      // Clear IndexedDB
      await this.deleteIndexedDB();

      // Clear sessionStorage
      sessionStorage.clear();

      console.log('All data cleared');
    } catch (error) {
      console.error('Clear data error:', error);
      throw error;
    }
  }

  /**
   * Delete IndexedDB
   */
  private async deleteIndexedDB(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.deleteDatabase('SanadDB');
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }

  /**
   * Get migration status
   */
  async getStatus(): Promise<MigrationStatus> {
    try {
      const currentVersion = this.getLastMigrationVersion();
      const lastAppVersion = this.getLastAppVersion();
      const backups = this.getAvailableBackups();

      return {
        currentMigrationVersion: currentVersion,
        targetMigrationVersion: MigrationService.CURRENT_MIGRATION_VERSION,
        lastAppVersion,
        availableBackups: backups,
        needsMigration: currentVersion < MigrationService.CURRENT_MIGRATION_VERSION,
      };
    } catch (error) {
      console.error('Error getting migration status:', error);
      throw error;
    }
  }

  /**
   * Get last migration version
   */
  private getLastMigrationVersion(): number {
    const version = localStorage.getItem(MigrationService.MIGRATION_VERSION_KEY);
    return version ? parseInt(version, 10) : 0;
  }

  /**
   * Set migration version
   */
  private setMigrationVersion(version: number): void {
    localStorage.setItem(MigrationService.MIGRATION_VERSION_KEY, version.toString());
  }

  /**
   * Get last app version
   */
  private getLastAppVersion(): string | null {
    return localStorage.getItem(MigrationService.LAST_APP_VERSION_KEY);
  }

  /**
   * Set last app version
   */
  private setLastAppVersion(version: string): void {
    localStorage.setItem(MigrationService.LAST_APP_VERSION_KEY, version);
  }
}

export default MigrationService.getInstance();
export type { MigrationStatus, BackupData };
