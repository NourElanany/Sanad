/**
 * E2E Test: Onboarding Flow
 * 
 * Tests the complete onboarding experience for new users:
 * - Welcome screens
 * - Permission requests
 * - Madhab selection
 * - Theme selection
 * - Completion and redirect to dashboard
 * 
 * **Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Onboarding Flow', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Clear all data to simulate first-time user
    await helpers.data.clearAllData();
    
    // Navigate to onboarding
    await page.goto('/onboarding');
  });

  test('should display welcome screen with Islamic branding', async ({ page }) => {
    // Check for Islamic-themed welcome message
    await expect(page.locator('h1')).toContainText(/مرحباً|Welcome/i);
    
    // Verify Islamic design elements are present
    const primaryColor = await page.locator('body').evaluate((el) => {
      return window.getComputedStyle(el).getPropertyValue('--primary-color');
    });
    
    // Should have Islamic color scheme (navy or emerald)
    expect(primaryColor).toBeTruthy();
  });

  test('should navigate through onboarding steps', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Step 1: Welcome screen
    await helpers.assert.assertVisible('h1');
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    
    // Step 2: Permissions screen
    await helpers.wait.waitForVisible('[data-testid="permissions-screen"]', 10000);
    
    // Skip permissions for testing (or mock them)
    await page.click('button:has-text("تخطي"), button:has-text("Skip")');
    
    // Step 3: Madhab selection
    await helpers.wait.waitForVisible('[data-testid="madhab-selection"]', 10000);
    
    // Select a madhab
    await page.click('[data-testid="madhab-hanafi"], [data-value="hanafi"]');
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    
    // Step 4: Theme selection
    await helpers.wait.waitForVisible('[data-testid="theme-selection"]', 10000);
    
    // Select light theme
    await page.click('[data-testid="theme-light"], [data-value="light"]');
    await page.click('button:has-text("إنهاء"), button:has-text("Finish")');
    
    // Should redirect to dashboard
    await page.waitForURL('/dashboard', { timeout: 10000 });
    await helpers.assert.assertURL(/\/dashboard/);
  });

  test('should save madhab preference', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate through to madhab selection
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    await page.click('button:has-text("تخطي"), button:has-text("Skip")');
    
    // Select Shafi'i madhab
    await page.click('[data-testid="madhab-shafii"], [data-value="shafii"]');
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    
    // Complete onboarding
    await page.click('[data-testid="theme-light"], [data-value="light"]');
    await page.click('button:has-text("إنهاء"), button:has-text("Finish")');
    
    // Wait for dashboard
    await page.waitForURL('/dashboard');
    
    // Check that madhab preference was saved
    const savedMadhab = await helpers.data.getLocalStorageItem('madhab');
    expect(savedMadhab).toBe('shafii');
  });

  test('should save theme preference', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate through to theme selection
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    await page.click('button:has-text("تخطي"), button:has-text("Skip")');
    await page.click('[data-testid="madhab-hanafi"], [data-value="hanafi"]');
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    
    // Select dark theme
    await page.click('[data-testid="theme-dark"], [data-value="dark"]');
    await page.click('button:has-text("إنهاء"), button:has-text("Finish")');
    
    // Wait for dashboard
    await page.waitForURL('/dashboard');
    
    // Check that theme preference was saved
    const savedTheme = await helpers.data.getLocalStorageItem('theme');
    expect(savedTheme).toBe('dark');
  });

  test('should not show onboarding again after completion', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Complete onboarding
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    await page.click('button:has-text("تخطي"), button:has-text("Skip")');
    await page.click('[data-testid="madhab-hanafi"], [data-value="hanafi"]');
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    await page.click('[data-testid="theme-light"], [data-value="light"]');
    await page.click('button:has-text("إنهاء"), button:has-text("Finish")');
    
    // Wait for dashboard
    await page.waitForURL('/dashboard');
    
    // Try to navigate to onboarding again
    await page.goto('/onboarding');
    
    // Should redirect to dashboard
    await page.waitForURL('/dashboard', { timeout: 5000 });
  });

  test('should handle back navigation', async ({ page }) => {
    // Go to second step
    await page.click('button:has-text("التالي"), button:has-text("Next")');
    
    // Go back
    await page.click('button:has-text("السابق"), button:has-text("Back")');
    
    // Should be back at welcome screen
    await expect(page.locator('h1')).toContainText(/مرحباً|Welcome/i);
  });

  test('should be responsive on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check that content is visible and properly sized
    const welcomeHeading = page.locator('h1');
    await expect(welcomeHeading).toBeVisible();
    
    // Check that buttons are accessible
    const nextButton = page.locator('button:has-text("التالي"), button:has-text("Next")');
    await expect(nextButton).toBeVisible();
    
    // Verify no horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });

  test('should support RTL layout for Arabic', async ({ page }) => {
    // Check if page has RTL direction
    const direction = await page.evaluate(() => {
      return document.documentElement.dir || document.body.dir;
    });
    
    // Should be RTL for Arabic content
    expect(direction).toBe('rtl');
  });
});
