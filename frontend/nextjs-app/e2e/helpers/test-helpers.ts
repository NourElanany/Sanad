/**
 * E2E Test Helper Functions
 * 
 * Provides reusable utilities for E2E tests including:
 * - Authentication helpers
 * - Navigation helpers
 * - Wait utilities
 * - Data setup/teardown
 */

import { Page, expect } from '@playwright/test';

/**
 * Authentication Helpers
 */
export class AuthHelpers {
  constructor(private page: Page) {}

  /**
   * Perform user login
   */
  async login(email: string, password: string) {
    await this.page.goto('/login');
    await this.page.fill('input[name="email"]', email);
    await this.page.fill('input[name="password"]', password);
    await this.page.click('button[type="submit"]');
    
    // Wait for redirect to dashboard
    await this.page.waitForURL('/dashboard', { timeout: 10000 });
  }

  /**
   * Perform user logout
   */
  async logout() {
    await this.page.click('[data-testid="user-menu"]');
    await this.page.click('[data-testid="logout-button"]');
    await this.page.waitForURL('/login');
  }

  /**
   * Check if user is authenticated
   */
  async isAuthenticated(): Promise<boolean> {
    const token = await this.page.evaluate(() => {
      return localStorage.getItem('access_token');
    });
    return !!token;
  }

  /**
   * Set authentication token directly (for test setup)
   */
  async setAuthToken(token: string) {
    await this.page.evaluate((token) => {
      localStorage.setItem('access_token', token);
    }, token);
  }
}

/**
 * Navigation Helpers
 */
export class NavigationHelpers {
  constructor(private page: Page) {}

  /**
   * Navigate to dashboard
   */
  async goToDashboard() {
    await this.page.goto('/dashboard');
    await this.page.waitForLoadState('networkidle');
  }

  /**
   * Navigate to Quran index
   */
  async goToQuran() {
    await this.page.goto('/quran');
    await this.page.waitForLoadState('networkidle');
  }

  /**
   * Navigate to AI Assistant
   */
  async goToAIAssistant() {
    await this.page.goto('/ai-assistant');
    await this.page.waitForLoadState('networkidle');
  }

  /**
   * Navigate to specific Surah
   */
  async goToSurah(surahNumber: number) {
    await this.page.goto(`/quran/mushaf/${surahNumber}`);
    await this.page.waitForLoadState('networkidle');
  }

  /**
   * Navigate to prayer times
   */
  async goToPrayerTimes() {
    await this.page.goto('/prayer-times');
    await this.page.waitForLoadState('networkidle');
  }
}

/**
 * Wait Utilities
 */
export class WaitHelpers {
  constructor(private page: Page) {}

  /**
   * Wait for element to be visible
   */
  async waitForVisible(selector: string, timeout = 5000) {
    await this.page.waitForSelector(selector, { 
      state: 'visible', 
      timeout 
    });
  }

  /**
   * Wait for text to appear
   */
  async waitForText(text: string, timeout = 5000) {
    await this.page.waitForSelector(`text=${text}`, { timeout });
  }

  /**
   * Wait for API response
   */
  async waitForAPIResponse(urlPattern: string | RegExp, timeout = 10000) {
    return await this.page.waitForResponse(
      response => {
        const url = response.url();
        if (typeof urlPattern === 'string') {
          return url.includes(urlPattern);
        }
        return urlPattern.test(url);
      },
      { timeout }
    );
  }

  /**
   * Wait for streaming response to complete
   */
  async waitForStreamingComplete(selector: string, timeout = 30000) {
    const startTime = Date.now();
    let previousText = '';
    let stableCount = 0;
    
    while (Date.now() - startTime < timeout) {
      const currentText = await this.page.textContent(selector) || '';
      
      if (currentText === previousText) {
        stableCount++;
        if (stableCount >= 3) {
          // Text hasn't changed for 3 checks, streaming likely complete
          return;
        }
      } else {
        stableCount = 0;
        previousText = currentText;
      }
      
      await this.page.waitForTimeout(500);
    }
    
    throw new Error('Streaming did not complete within timeout');
  }
}

/**
 * Offline Mode Helpers
 */
export class OfflineHelpers {
  constructor(private page: Page) {}

  /**
   * Enable offline mode
   */
  async goOffline() {
    await this.page.context().setOffline(true);
  }

  /**
   * Disable offline mode
   */
  async goOnline() {
    await this.page.context().setOffline(false);
  }

  /**
   * Check if offline indicator is visible
   */
  async isOfflineIndicatorVisible(): Promise<boolean> {
    const indicator = await this.page.locator('[data-testid="offline-indicator"]');
    return await indicator.isVisible();
  }

  /**
   * Wait for service worker to be ready
   */
  async waitForServiceWorker() {
    await this.page.evaluate(async () => {
      if ('serviceWorker' in navigator) {
        await navigator.serviceWorker.ready;
      }
    });
  }
}

/**
 * Data Helpers
 */
export class DataHelpers {
  constructor(private page: Page) {}

  /**
   * Clear all local storage
   */
  async clearLocalStorage() {
    await this.page.evaluate(() => {
      localStorage.clear();
    });
  }

  /**
   * Clear all session storage
   */
  async clearSessionStorage() {
    await this.page.evaluate(() => {
      sessionStorage.clear();
    });
  }

  /**
   * Clear all cookies
   */
  async clearCookies() {
    await this.page.context().clearCookies();
  }

  /**
   * Clear all browser data
   */
  async clearAllData() {
    await this.clearLocalStorage();
    await this.clearSessionStorage();
    await this.clearCookies();
  }

  /**
   * Get local storage item
   */
  async getLocalStorageItem(key: string): Promise<string | null> {
    return await this.page.evaluate((key) => {
      return localStorage.getItem(key);
    }, key);
  }

  /**
   * Set local storage item
   */
  async setLocalStorageItem(key: string, value: string) {
    await this.page.evaluate(({ key, value }) => {
      localStorage.setItem(key, value);
    }, { key, value });
  }
}

/**
 * Assertion Helpers
 */
export class AssertionHelpers {
  constructor(private page: Page) {}

  /**
   * Assert element is visible
   */
  async assertVisible(selector: string) {
    await expect(this.page.locator(selector)).toBeVisible();
  }

  /**
   * Assert element contains text
   */
  async assertContainsText(selector: string, text: string) {
    await expect(this.page.locator(selector)).toContainText(text);
  }

  /**
   * Assert URL matches
   */
  async assertURL(url: string | RegExp) {
    await expect(this.page).toHaveURL(url);
  }

  /**
   * Assert element count
   */
  async assertCount(selector: string, count: number) {
    await expect(this.page.locator(selector)).toHaveCount(count);
  }
}

/**
 * Create all helpers for a page
 */
export function createHelpers(page: Page) {
  return {
    auth: new AuthHelpers(page),
    nav: new NavigationHelpers(page),
    wait: new WaitHelpers(page),
    offline: new OfflineHelpers(page),
    data: new DataHelpers(page),
    assert: new AssertionHelpers(page),
  };
}
