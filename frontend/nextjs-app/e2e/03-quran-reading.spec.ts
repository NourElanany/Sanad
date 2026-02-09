/**
 * E2E Test: Quran Reading Flow
 * 
 * Tests the complete Quran reading experience:
 * - Quran index navigation
 * - Surah/Juz browsing
 * - Mushaf page view
 * - Verse interactions
 * - Tafsir access
 * - Bookmarks
 * 
 * **Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5, 6.1, 6.2, 6.3, 6.4, 6.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Quran Reading Flow', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to Quran index
    await helpers.nav.goToQuran();
  });

  test('should display Quran index with surahs', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for Quran index heading
    await helpers.assert.assertVisible('h1');
    await expect(page.locator('h1')).toContainText(/القرآن|Quran/i);
    
    // Should display list of surahs
    await helpers.assert.assertVisible('[data-testid="surah-list"]');
    
    // Should have at least 114 surahs
    const surahCount = await page.locator('[data-testid^="surah-"]').count();
    expect(surahCount).toBeGreaterThanOrEqual(114);
  });

  test('should display surah information', async ({ page }) => {
    // Check first surah (Al-Fatiha)
    const firstSurah = page.locator('[data-testid="surah-1"]');
    await expect(firstSurah).toBeVisible();
    
    // Should show surah name
    await expect(firstSurah.locator('[data-testid="surah-name"]')).toContainText(/الفاتحة|Al-Fatiha/i);
    
    // Should show verse count
    await expect(firstSurah.locator('[data-testid="verse-count"]')).toContainText(/7/);
  });

  test('should have tabs for Surahs, Juz, and Bookmarks', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for tabs
    await helpers.assert.assertVisible('[data-testid="tab-surahs"]');
    await helpers.assert.assertVisible('[data-testid="tab-juz"]');
    await helpers.assert.assertVisible('[data-testid="tab-bookmarks"]');
  });

  test('should switch between tabs', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click Juz tab
    await page.click('[data-testid="tab-juz"]');
    
    // Should show Juz list
    await helpers.wait.waitForVisible('[data-testid="juz-list"]');
    
    // Click back to Surahs tab
    await page.click('[data-testid="tab-surahs"]');
    
    // Should show Surah list again
    await helpers.wait.waitForVisible('[data-testid="surah-list"]');
  });

  test('should search for surahs', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Type in search box
    await page.fill('[data-testid="quran-search"]', 'البقرة');
    
    // Should filter results
    await page.waitForTimeout(500); // Wait for debounce
    
    // Should show Al-Baqarah
    await expect(page.locator('[data-testid="surah-2"]')).toBeVisible();
    
    // Other surahs should be hidden or filtered
    const visibleSurahs = await page.locator('[data-testid^="surah-"]:visible').count();
    expect(visibleSurahs).toBeLessThan(114);
  });

  test('should navigate to surah page', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Click on Al-Fatiha
    await page.click('[data-testid="surah-1"]');
    
    // Should navigate to mushaf view
    await page.waitForURL(/\/quran\/mushaf/, { timeout: 5000 });
    
    // Should display Quranic text
    await helpers.wait.waitForVisible('[data-testid="quran-text"]');
  });

  test('should display mushaf page with Quranic text', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a specific page
    await helpers.nav.goToSurah(1);
    
    // Should show Quranic text
    await helpers.assert.assertVisible('[data-testid="quran-text"]');
    
    // Text should be in Arabic
    const quranText = await page.locator('[data-testid="quran-text"]').textContent();
    expect(quranText).toMatch(/[\u0600-\u06FF]/); // Arabic Unicode range
  });

  test('should navigate between pages', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to page 1
    await helpers.nav.goToSurah(1);
    
    // Should have next button
    await helpers.assert.assertVisible('[data-testid="next-page"]');
    
    // Click next
    await page.click('[data-testid="next-page"]');
    
    // URL should change
    await page.waitForTimeout(500);
    const currentURL = page.url();
    expect(currentURL).toContain('/quran/mushaf/');
  });

  test('should tap verse to show options', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Wait for verses to load
    await helpers.wait.waitForVisible('[data-testid^="verse-"]');
    
    // Click on a verse
    await page.click('[data-testid="verse-1"]');
    
    // Should show verse options modal
    await helpers.wait.waitForVisible('[data-testid="verse-options-modal"]');
    
    // Should have tafsir option
    await expect(page.locator('[data-testid="option-tafsir"]')).toBeVisible();
    
    // Should have audio option
    await expect(page.locator('[data-testid="option-audio"]')).toBeVisible();
    
    // Should have translation option
    await expect(page.locator('[data-testid="option-translation"]')).toBeVisible();
  });

  test('should open tafsir viewer', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Click on a verse
    await page.click('[data-testid="verse-1"]');
    
    // Click tafsir option
    await page.click('[data-testid="option-tafsir"]');
    
    // Should show tafsir content
    await helpers.wait.waitForVisible('[data-testid="tafsir-content"]');
    
    // Should have tafsir text
    const tafsirText = await page.locator('[data-testid="tafsir-content"]').textContent();
    expect(tafsirText).toBeTruthy();
  });

  test('should add bookmark', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Click bookmark button
    await page.click('[data-testid="bookmark-button"]');
    
    // Should show success message or change icon
    await page.waitForTimeout(500);
    
    // Navigate back to Quran index
    await helpers.nav.goToQuran();
    
    // Go to bookmarks tab
    await page.click('[data-testid="tab-bookmarks"]');
    
    // Should show the bookmark
    await helpers.wait.waitForVisible('[data-testid="bookmark-list"]');
    const bookmarkCount = await page.locator('[data-testid^="bookmark-"]').count();
    expect(bookmarkCount).toBeGreaterThan(0);
  });

  test('should display translation when enabled', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Enable translation
    await page.click('[data-testid="toggle-translation"]');
    
    // Should show translation text
    await helpers.wait.waitForVisible('[data-testid="translation-text"]');
    
    // Translation should be in English or another language
    const translationText = await page.locator('[data-testid="translation-text"]').textContent();
    expect(translationText).toBeTruthy();
  });

  test('should adjust font size', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Get initial font size
    const initialFontSize = await page.locator('[data-testid="quran-text"]').evaluate((el) => {
      return window.getComputedStyle(el).fontSize;
    });
    
    // Click increase font size
    await page.click('[data-testid="increase-font"]');
    
    // Wait for change
    await page.waitForTimeout(300);
    
    // Get new font size
    const newFontSize = await page.locator('[data-testid="quran-text"]').evaluate((el) => {
      return window.getComputedStyle(el).fontSize;
    });
    
    // Font size should have increased
    expect(parseFloat(newFontSize)).toBeGreaterThan(parseFloat(initialFontSize));
  });

  test('should save reading position', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a specific surah
    await helpers.nav.goToSurah(2); // Al-Baqarah
    
    // Wait for page to load
    await helpers.wait.waitForVisible('[data-testid="quran-text"]');
    
    // Reading position should be saved in localStorage
    const savedPosition = await helpers.data.getLocalStorageItem('last_read_position');
    expect(savedPosition).toBeTruthy();
  });

  test('should be responsive on mobile', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Navigate to Quran index
    await helpers.nav.goToQuran();
    
    // Surah list should be visible
    await expect(page.locator('[data-testid="surah-list"]')).toBeVisible();
    
    // Navigate to a surah
    await page.click('[data-testid="surah-1"]');
    
    // Quran text should be readable
    await expect(page.locator('[data-testid="quran-text"]')).toBeVisible();
    
    // No horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });

  test('should use proper Quranic font', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Check font family
    const fontFamily = await page.locator('[data-testid="quran-text"]').evaluate((el) => {
      return window.getComputedStyle(el).fontFamily;
    });
    
    // Should use Quranic font (KFGQPC Uthman Taha Naskh or similar)
    expect(fontFamily.toLowerCase()).toMatch(/uthman|amiri|naskh|quran/i);
  });

  test('should handle RTL text direction', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to a surah
    await helpers.nav.goToSurah(1);
    
    // Check text direction
    const direction = await page.locator('[data-testid="quran-text"]').evaluate((el) => {
      return window.getComputedStyle(el).direction;
    });
    
    // Should be RTL for Arabic
    expect(direction).toBe('rtl');
  });
});
