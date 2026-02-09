// Update service for Next.js web app
// Handles service worker updates and version checking

export interface UpdateInfo {
  version: string;
  releaseNotes: string;
  isMandatory: boolean;
  releaseDate: string;
  features: string[];
  bugFixes: string[];
}

class UpdateService {
  private static instance: UpdateService;
  private currentVersion: string = process.env.NEXT_PUBLIC_APP_VERSION || '1.0.0';
  private updateCheckInterval: NodeJS.Timeout | null = null;
  private serviceWorkerRegistration: ServiceWorkerRegistration | null = null;

  private constructor() {}

  static getInstance(): UpdateService {
    if (!UpdateService.instance) {
      UpdateService.instance = new UpdateService();
    }
    return UpdateService.instance;
  }

  /**
   * Initialize the update service
   */
  async initialize(): Promise<void> {
    if (typeof window === 'undefined') return;

    // Register service worker
    if ('serviceWorker' in navigator) {
      try {
        this.serviceWorkerRegistration = await navigator.serviceWorker.register('/sw.js');
        
        // Listen for updates
        this.serviceWorkerRegistration.addEventListener('updatefound', () => {
          const newWorker = this.serviceWorkerRegistration?.installing;
          
          if (newWorker) {
            newWorker.addEventListener('statechange', () => {
              if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
                // New service worker available
                this.notifyUpdateAvailable();
              }
            });
          }
        });
      } catch (error) {
        console.error('Service worker registration failed:', error);
      }
    }

    // Start periodic update checks
    this.startUpdateChecks();
  }

  /**
   * Check for updates from the server
   */
  async checkForUpdates(): Promise<UpdateInfo | null> {
    try {
      const response = await fetch('/api/updates/check', {
        headers: {
          'X-App-Version': this.currentVersion,
          'X-Platform': 'web',
        },
      });

      if (response.ok) {
        const data: UpdateInfo = await response.json();
        
        if (this.isNewerVersion(this.currentVersion, data.version)) {
          return data;
        }
      }

      return null;
    } catch (error) {
      console.error('Error checking for updates:', error);
      return null;
    }
  }

  /**
   * Start periodic update checks
   */
  private startUpdateChecks(): void {
    // Check for updates every 30 minutes
    this.updateCheckInterval = setInterval(async () => {
      const update = await this.checkForUpdates();
      if (update) {
        this.notifyUpdateAvailable(update);
      }
    }, 30 * 60 * 1000);
  }

  /**
   * Stop periodic update checks
   */
  stopUpdateChecks(): void {
    if (this.updateCheckInterval) {
      clearInterval(this.updateCheckInterval);
      this.updateCheckInterval = null;
    }
  }

  /**
   * Notify user about available update
   */
  private notifyUpdateAvailable(updateInfo?: UpdateInfo): void {
    // Dispatch custom event that components can listen to
    const event = new CustomEvent('app-update-available', {
      detail: updateInfo,
    });
    window.dispatchEvent(event);
  }

  /**
   * Apply the update (reload the page)
   */
  async applyUpdate(): Promise<void> {
    if (this.serviceWorkerRegistration?.waiting) {
      // Tell the service worker to skip waiting
      this.serviceWorkerRegistration.waiting.postMessage({ type: 'SKIP_WAITING' });
      
      // Reload the page after a short delay
      setTimeout(() => {
        window.location.reload();
      }, 100);
    } else {
      // Just reload the page
      window.location.reload();
    }
  }

  /**
   * Compare version strings
   */
  private isNewerVersion(current: string, latest: string): boolean {
    const currentParts = current.split('.').map(Number);
    const latestParts = latest.split('.').map(Number);

    for (let i = 0; i < 3; i++) {
      if (latestParts[i] > currentParts[i]) return true;
      if (latestParts[i] < currentParts[i]) return false;
    }

    return false;
  }

  /**
   * Get current version
   */
  getCurrentVersion(): string {
    return this.currentVersion;
  }
}

export default UpdateService.getInstance();
