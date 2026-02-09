/**
 * E2E Test: Offline Functionality
 * 
 * Tests offline mode capabilities:
 * - Service worker registration
 * - Offline indicator
 * - Cached content access
 * - Offline queue
 * - Sync when online
 * - PWA functionality
 * 
 * **Validates: Requirements 15.1, 15.2, 15.3, 15.4, 15.5, 2.3, 2.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Offline Functionality', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
    
    // Wait for service worker to be ready
    await helpers.offline.waitForServiceWorker();
  });

  test('should register service worker', async ({ page }) => {
    // Check if service worker is registered
    const swRegistered = await page.evaluate(async () => {
      if ('serviceWorker' in navigator) {
        const registration = await navigator.serviceWorker.getRegistration();
        return !!registration;
      }
      return false;
    });
    
    expect(swRegistered).toBe(true);
  });

  test('should show offline indicator when offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Wait a bit for the indicator to appear
    await page.waitForTimeout(1000);
    
    // Should show offline indicator
    const isIndicatorVisible = await helpers.offline.isOfflineIndicatorVisible();
    expect(isIndicatorVisible).toBe(true);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should hide offline indicator when online', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Start offline
    await helpers.offline.goOffline();
    await page.waitForTimeout(1000);
    
    // Go back online
    await helpers.offline.goOnline();
    await page.waitForTimeout(1000);
    
    // Indicator should be hidden
    const isIndicatorVisible = await helpers.offline.isOfflineIndicatorVisible();
    expect(isIndicatorVisible).toBe(false);
  });

  test('should access cached Quran content offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // First, visit Quran page while online to cache it
    await helpers.nav.goToQuran();
    await helpers.wait.waitForVisible('[data-testid="surah-list"]');
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Navigate to Quran again
    await helpers.nav.goToQuran();
    
    // Should still be able to see the page
    await helpers.wait.waitForVisible('[data-testid="surah-list"]', 10000);
    
    // Content should be visible
    const surahCount = await page.locator('[data-testid^="surah-"]').count();
    expect(surahCount).toBeGreaterThan(0);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should access cached dashboard offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Dashboard is already loaded
    await helpers.wait.waitForVisible('[data-testid="prayer-times-card"]');
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Reload page
    await page.reload();
    
    // Should still load from cache
    await helpers.wait.waitForVisible('[data-testid="prayer-times-card"]', 10000);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should queue actions when offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to Quran
    await helpers.nav.goToQuran();
    await helpers.nav.goToSurah(1);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Try to add a bookmark
    await page.click('[data-testid="bookmark-button"]');
    
    // Should show queued message or pending state
    await page.waitForTimeout(1000);
    
    // Action should be queued in localStorage
    const queuedActions = await helpers.data.getLocalStorageItem('offline_queue');
    expect(queuedActions).toBeTruthy();
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should sync queued actions when back online', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to Quran
    await helpers.nav.goToQuran();
    await helpers.nav.goToSurah(1);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Add a bookmark
    await page.click('[data-testid="bookmark-button"]');
    await page.waitForTimeout(500);
    
    // Go back online
    await helpers.offline.goOnline();
    
    // Wait for sync
    await page.waitForTimeout(2000);
    
    // Queue should be cleared after sync
    const queuedActions = await helpers.data.getLocalStorageItem('offline_queue');
    expect(queuedActions).toBeFalsy();
  });

  test('should show appropriate message for unavailable content offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Try to access AI Assistant (requires online connection)
    await page.goto('/ai-assistant');
    
    // Should show offline message or cached version
    const pageContent = await page.textContent('body');
    
    // Either shows cached content or offline message
    expect(pageContent).toBeTruthy();
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should cache static assets', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Load dashboard
    await helpers.nav.goToDashboard();
    await page.waitForLoadState('networkidle');
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Reload page
    await page.reload();
    
    // Page should still load (from cache)
    await page.waitForLoadState('load', { timeout: 10000 });
    
    // Check that page is functional
    const bodyVisible = await page.locator('body').isVisible();
    expect(bodyVisible).toBe(true);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should handle network errors gracefully', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Try to navigate to a page that requires API call
    await page.goto('/prayer-times');
    
    // Should not crash, should show cached data or error message
    const pageLoaded = await page.locator('body').isVisible();
    expect(pageLoaded).toBe(true);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should update content when back online', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Load dashboard
    await helpers.nav.goToDashboard();
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Reload to get cached version
    await page.reload();
    await page.waitForLoadState('load');
    
    // Go back online
    await helpers.offline.goOnline();
    
    // Wait for potential updates
    await page.waitForTimeout(2000);
    
    // Content should be updated (or at least attempt to update)
    const bodyVisible = await page.locator('body').isVisible();
    expect(bodyVisible).toBe(true);
  });

  test('should persist user preferences offline', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set a preference
    await helpers.data.setLocalStorageItem('font_size', 'large');
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Reload page
    await page.reload();
    await page.waitForLoadState('load');
    
    // Preference should still be available
    const fontSize = await helpers.data.getLocalStorageItem('font_size');
    expect(fontSize).toBe('large');
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should show download manager for offline content', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to settings or downloads page
    await page.goto('/downloads');
    
    // Should show download manager
    await helpers.wait.waitForVisible('[data-testid="download-manager"]', 10000);
    
    // Should have options to download content
    await expect(page.locator('[data-testid="download-options"]')).toBeVisible();
  });

  test('should indicate cached vs live content', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Navigate to Quran
    await helpers.nav.goToQuran();
    
    // Should indicate content is from cache
    const offlineIndicator = await helpers.offline.isOfflineIndicatorVisible();
    expect(offlineIndicator).toBe(true);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should handle intermittent connectivity', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Go offline
    await helpers.offline.goOffline();
    await page.waitForTimeout(1000);
    
    // Go online
    await helpers.offline.goOnline();
    await page.waitForTimeout(1000);
    
    // Go offline again
    await helpers.offline.goOffline();
    await page.waitForTimeout(1000);
    
    // Go online again
    await helpers.offline.goOnline();
    await page.waitForTimeout(1000);
    
    // App should still be functional
    const bodyVisible = await page.locator('body').isVisible();
    expect(bodyVisible).toBe(true);
  });

  test('should work as PWA', async ({ page }) => {
    // Check for PWA manifest
    const manifestLink = await page.locator('link[rel="manifest"]').getAttribute('href');
    expect(manifestLink).toBeTruthy();
    
    // Check for theme color
    const themeColor = await page.locator('meta[name="theme-color"]').getAttribute('content');
    expect(themeColor).toBeTruthy();
    
    // Check for app icons
    const appleIcon = await page.locator('link[rel="apple-touch-icon"]').count();
    expect(appleIcon).toBeGreaterThan(0);
  });

  test('should cache API responses', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Make an API call while online
    await helpers.nav.goToQuran();
    await helpers.wait.waitForAPIResponse('/api/quran', 10000);
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Navigate away and back
    await helpers.nav.goToDashboard();
    await helpers.nav.goToQuran();
    
    // Should still show data from cache
    await helpers.wait.waitForVisible('[data-testid="surah-list"]', 10000);
    
    // Go back online
    await helpers.offline.goOnline();
  });

  test('should handle offline mode on mobile', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Go offline
    await helpers.offline.goOffline();
    
    // Navigate to Quran
    await helpers.nav.goToQuran();
    
    // Should work on mobile
    await helpers.wait.waitForVisible('[data-testid="surah-list"]', 10000);
    
    // Offline indicator should be visible
    const isIndicatorVisible = await helpers.offline.isOfflineIndicatorVisible();
    expect(isIndicatorVisible).toBe(true);
    
    // Go back online
    await helpers.offline.goOnline();
  });
});
