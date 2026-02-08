/**
 * Connectivity service for monitoring network status in Next.js
 */
class ConnectivityService {
  private listeners: Set<(status: ConnectivityStatus) => void> = new Set();
  private currentStatus: ConnectivityStatus = ConnectivityStatus.UNKNOWN;

  constructor() {
    if (typeof window !== 'undefined') {
      this.init();
    }
  }

  /**
   * Initialize connectivity monitoring
   */
  private init(): void {
    // Set initial status
    this.updateStatus();

    // Listen to online/offline events
    window.addEventListener('online', () => {
      console.log('📡 Network: Online');
      this.updateStatus();
    });

    window.addEventListener('offline', () => {
      console.log('📡 Network: Offline');
      this.updateStatus();
    });

    // Listen to connection change events (if supported)
    if ('connection' in navigator) {
      const connection = (navigator as any).connection;
      connection?.addEventListener('change', () => {
        console.log('📡 Network: Connection changed');
        this.updateStatus();
      });
    }

    console.log('📡 Connectivity service initialized');
  }

  /**
   * Update connectivity status
   */
  private updateStatus(): void {
    const newStatus = navigator.onLine 
      ? ConnectivityStatus.CONNECTED 
      : ConnectivityStatus.DISCONNECTED;

    if (this.currentStatus !== newStatus) {
      this.currentStatus = newStatus;
      this.notifyListeners(newStatus);
    }
  }

  /**
   * Notify all listeners of status change
   */
  private notifyListeners(status: ConnectivityStatus): void {
    this.listeners.forEach(listener => {
      try {
        listener(status);
      } catch (error) {
        console.error('❌ Error notifying connectivity listener:', error);
      }
    });
  }

  /**
   * Get current connectivity status
   */
  getCurrentStatus(): ConnectivityStatus {
    return this.currentStatus;
  }

  /**
   * Check if device is connected to internet
   */
  async isConnected(): Promise<boolean> {
    if (typeof window === 'undefined') return true; // Assume connected on server
    
    // Quick check using navigator.onLine
    if (!navigator.onLine) {
      return false;
    }

    // Additional check by trying to fetch a small resource
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 3000);

      await fetch('/api/health', {
        method: 'HEAD',
        signal: controller.signal,
      });

      clearTimeout(timeoutId);
      return true;
    } catch (error) {
      return false;
    }
  }

  /**
   * Subscribe to connectivity changes
   */
  subscribe(listener: (status: ConnectivityStatus) => void): () => void {
    this.listeners.add(listener);

    // Return unsubscribe function
    return () => {
      this.listeners.delete(listener);
    };
  }

  /**
   * Get connection type (if available)
   */
  getConnectionType(): string {
    if (typeof window === 'undefined') return 'unknown';

    const connection = (navigator as any).connection;
    if (connection) {
      return connection.effectiveType || connection.type || 'unknown';
    }

    return 'unknown';
  }

  /**
   * Check if connection is slow
   */
  isSlowConnection(): boolean {
    if (typeof window === 'undefined') return false;

    const connection = (navigator as any).connection;
    if (connection) {
      const effectiveType = connection.effectiveType;
      return effectiveType === 'slow-2g' || effectiveType === '2g';
    }

    return false;
  }

  /**
   * Get estimated downlink speed (Mbps)
   */
  getDownlinkSpeed(): number | null {
    if (typeof window === 'undefined') return null;

    const connection = (navigator as any).connection;
    return connection?.downlink || null;
  }
}

/**
 * Connectivity status enum
 */
export enum ConnectivityStatus {
  CONNECTED = 'connected',
  DISCONNECTED = 'disconnected',
  UNKNOWN = 'unknown',
}

/**
 * Helper functions for connectivity status
 */
export const connectivityHelpers = {
  isConnected: (status: ConnectivityStatus) => status === ConnectivityStatus.CONNECTED,
  isDisconnected: (status: ConnectivityStatus) => status === ConnectivityStatus.DISCONNECTED,
  
  getMessage: (status: ConnectivityStatus): string => {
    switch (status) {
      case ConnectivityStatus.CONNECTED:
        return 'Connected to internet';
      case ConnectivityStatus.DISCONNECTED:
        return 'No internet connection';
      case ConnectivityStatus.UNKNOWN:
        return 'Checking connection...';
    }
  },

  getIcon: (status: ConnectivityStatus): string => {
    switch (status) {
      case ConnectivityStatus.CONNECTED:
        return '✅';
      case ConnectivityStatus.DISCONNECTED:
        return '❌';
      case ConnectivityStatus.UNKNOWN:
        return '❓';
    }
  },
};

// Export singleton instance
export const connectivityService = new ConnectivityService();
