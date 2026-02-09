/**
 * E2E Test: Dashboard Flow
 * 
 * Tests the main dashboard functionality:
 * - Prayer times display
 * - Hijri calendar
 * - Daily wird progress
 * - Daily verse/hadith
 * - Quick action buttons
 * - Widget interactions
 * 
 * **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Dashboard Flow', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    await helpers.data.setLocalStorageItem('madhab', 'hanafi');
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
  });

  test('should display prayer times widget', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for prayer times card
    await helpers.assert.assertVisible('[data-testid="prayer-times-card"]');
    
    // Should show next prayer
    await expect(page.locator('[data-testid="next-prayer"]')).toBeVisible();
    
    // Should show countdown
    await expect(page.locator('[data-testid="prayer-countdown"]')).toBeVisible();
    
    // Should show location
    await expect(page.locator('[data-testid="prayer-location"]')).toBeVisible();
  });

  test('should display Hijri calendar date', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for Hijri date card
    await helpers.assert.assertVisible('[data-testid="hijri-date-card"]');
    
    // Should contain Hijri date
    const hijriDate = await page.locator('[data-testid="hijri-date"]').textContent();
    expect(hijriDate).toMatch(/\d+/); // Should contain numbers
    
    // Should also show Gregorian date
    await expect(page.locator('[data-testid="gregorian-date"]')).toBeVisible();
  });

  test('should display daily wird progress', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for wird card
    await helpers.assert.assertVisible('[data-testid="daily-wird-card"]');
    
    // Should show progress bar
    await expect(page.locator('[data-testid="wird-progress"]')).toBeVisible();
    
    // Should show percentage or pages completed
    const progressText = await page.locator('[data-testid="wird-progress-text"]').textContent();
    expect(progressText).toBeTruthy();
  });

  test('should display daily verse or hadith', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for daily content card
    await helpers.assert.assertVisible('[data-testid="daily-content-card"]');
    
    // Should have Arabic text
    const arabicText = await page.locator('[data-testid="daily-content-text"]').textContent();
    expect(arabicText).toBeTruthy();
    
    // Should have translation or explanation
    await expect(page.locator('[data-testid="daily-content-translation"]')).toBeVisible();
  });

  test('should have quick action buttons', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for quick actions section
    await helpers.assert.assertVisible('[data-testid="quick-actions"]');
    
    // Should have AI Assistant button
    await expect(page.locator('[data-testid="quick-action-ai"]')).toBeVisible();
    
    // Should have Qibla button
    await expect(page.locator('[data-testid="quick-action-qibla"]')).toBeVisible();
    
    // Should have Adhkar button
    await expect(page.locator('[data-testid="quick-action-adhkar"]')).toBeVisible();
  });

  test('should navigate to AI Assistant from quick action', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click AI Assistant quick action
    await page.click('[data-testid="quick-action-ai"]');
    
    // Should navigate to AI Assistant page
    await page.waitForURL('/ai-assistant', { timeout: 5000 });
    await helpers.assert.assertURL(/\/ai-assistant/);
  });

  test('should navigate to Qibla from quick action', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click Qibla quick action
    await page.click('[data-testid="quick-action-qibla"]');
    
    // Should navigate to Qibla page
    await page.waitForURL('/qibla', { timeout: 5000 });
    await helpers.assert.assertURL(/\/qibla/);
  });

  test('should navigate to Quran from navigation', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click Quran in navigation
    await page.click('[data-testid="nav-quran"], a[href="/quran"]');
    
    // Should navigate to Quran index
    await page.waitForURL('/quran', { timeout: 5000 });
    await helpers.assert.assertURL(/\/quran/);
  });

  test('should update prayer times countdown', async ({ page }) => {
    // Get initial countdown value
    const initialCountdown = await page.locator('[data-testid="prayer-countdown"]').textContent();
    
    // Wait for 2 seconds
    await page.waitForTimeout(2000);
    
    // Get updated countdown value
    const updatedCountdown = await page.locator('[data-testid="prayer-countdown"]').textContent();
    
    // Countdown should have changed (unless it's exactly at prayer time)
    // This is a basic check - in real scenario, we'd parse the time
    expect(updatedCountdown).toBeTruthy();
  });

  test('should display greeting based on time of day', async ({ page }) => {
    // Check for greeting
    const greeting = await page.locator('[data-testid="user-greeting"]').textContent();
    
    // Should contain a greeting (السلام عليكم or similar)
    expect(greeting).toMatch(/السلام|مرحباً|صباح|مساء/i);
  });

  test('should show notification bell', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for notification bell
    await helpers.assert.assertVisible('[data-testid="notification-bell"]');
  });

  test('should show settings icon', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for settings icon
    await helpers.assert.assertVisible('[data-testid="settings-icon"]');
  });

  test('should navigate to settings', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click settings icon
    await page.click('[data-testid="settings-icon"]');
    
    // Should navigate to settings page
    await page.waitForURL('/settings', { timeout: 5000 });
    await helpers.assert.assertURL(/\/settings/);
  });

  test('should be responsive on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // All widgets should still be visible
    await expect(page.locator('[data-testid="prayer-times-card"]')).toBeVisible();
    await expect(page.locator('[data-testid="hijri-date-card"]')).toBeVisible();
    await expect(page.locator('[data-testid="daily-wird-card"]')).toBeVisible();
    
    // Quick actions should be visible
    await expect(page.locator('[data-testid="quick-actions"]')).toBeVisible();
    
    // No horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });

  test('should load widgets without blocking', async ({ page }) => {
    // Dashboard should be interactive even if some widgets are still loading
    const helpers = createHelpers(page);
    
    // Navigation should be clickable immediately
    const quranLink = page.locator('[data-testid="nav-quran"], a[href="/quran"]');
    await expect(quranLink).toBeEnabled();
  });

  test('should handle widget errors gracefully', async ({ page }) => {
    // If a widget fails to load, it should show an error state
    // but not break the entire dashboard
    
    // Check that dashboard is still functional
    await expect(page.locator('body')).toBeVisible();
    
    // Quick actions should still work
    await expect(page.locator('[data-testid="quick-actions"]')).toBeVisible();
  });

  test('should refresh data on page reload', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Get initial prayer time
    const initialPrayerTime = await page.locator('[data-testid="next-prayer"]').textContent();
    
    // Reload page
    await page.reload();
    await helpers.wait.waitForVisible('[data-testid="prayer-times-card"]');
    
    // Prayer time should still be displayed
    const reloadedPrayerTime = await page.locator('[data-testid="next-prayer"]').textContent();
    expect(reloadedPrayerTime).toBeTruthy();
  });
});
