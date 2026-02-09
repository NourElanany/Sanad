/**
 * E2E Test: Authentication Flow
 * 
 * Tests user authentication functionality:
 * - Login flow
 * - Logout flow
 * - Token management
 * - Protected routes
 * - Session persistence
 * - Error handling
 * 
 * **Validates: Requirements 14.1, 14.2, 14.4, 14.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Authentication Flow', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Clear all data to start fresh
    await helpers.data.clearAllData();
  });

  test('should redirect to login when not authenticated', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Try to access protected route
    await page.goto('/dashboard');
    
    // Should redirect to login
    await page.waitForURL('/login', { timeout: 5000 });
    await helpers.assert.assertURL(/\/login/);
  });

  test('should display login form', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to login
    await page.goto('/login');
    
    // Should show login form
    await helpers.assert.assertVisible('[data-testid="login-form"]');
    
    // Should have email input
    await helpers.assert.assertVisible('input[name="email"]');
    
    // Should have password input
    await helpers.assert.assertVisible('input[name="password"]');
    
    // Should have submit button
    await helpers.assert.assertVisible('button[type="submit"]');
  });

  test('should show validation errors for empty fields', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to login
    await page.goto('/login');
    
    // Click submit without filling fields
    await page.click('button[type="submit"]');
    
    // Should show validation errors
    await helpers.wait.waitForVisible('[data-testid="email-error"]', 5000);
    await helpers.wait.waitForVisible('[data-testid="password-error"]', 5000);
  });

  test('should show error for invalid email format', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Navigate to login
    await page.goto('/login');
    
    // Enter invalid email
    await page.fill('input[name="email"]', 'invalid-email');
    await page.fill('input[name="password"]', 'password123');
    
    // Click submit
    await page.click('button[type="submit"]');
    
    // Should show email format error
    await helpers.wait.waitForVisible('[data-testid="email-error"]', 5000);
  });

  test('should login with valid credentials', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock successful login response
    await page.route('**/api/auth/login', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          access_token: 'mock-access-token',
          refresh_token: 'mock-refresh-token',
          user: {
            id: '1',
            email: 'test@example.com',
            name: 'Test User'
          }
        })
      });
    });
    
    // Navigate to login
    await page.goto('/login');
    
    // Fill in credentials
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Should redirect to dashboard
    await page.waitForURL('/dashboard', { timeout: 10000 });
    await helpers.assert.assertURL(/\/dashboard/);
  });

  test('should store auth tokens after login', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock successful login
    await page.route('**/api/auth/login', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          access_token: 'mock-access-token',
          refresh_token: 'mock-refresh-token',
          user: { id: '1', email: 'test@example.com' }
        })
      });
    });
    
    // Login
    await page.goto('/login');
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');
    
    // Wait for redirect
    await page.waitForURL('/dashboard');
    
    // Check that tokens are stored
    const accessToken = await helpers.data.getLocalStorageItem('access_token');
    const refreshToken = await helpers.data.getLocalStorageItem('refresh_token');
    
    expect(accessToken).toBe('mock-access-token');
    expect(refreshToken).toBe('mock-refresh-token');
  });

  test('should show error for invalid credentials', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock failed login response
    await page.route('**/api/auth/login', route => {
      route.fulfill({
        status: 401,
        contentType: 'application/json',
        body: JSON.stringify({
          error: 'Invalid credentials'
        })
      });
    });
    
    // Navigate to login
    await page.goto('/login');
    
    // Fill in credentials
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'wrongpassword');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Should show error message
    await helpers.wait.waitForVisible('[data-testid="login-error"]', 5000);
    
    // Should still be on login page
    await helpers.assert.assertURL(/\/login/);
  });

  test('should logout successfully', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('access_token', 'mock-token');
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
    
    // Click user menu
    await page.click('[data-testid="user-menu"]');
    
    // Click logout
    await page.click('[data-testid="logout-button"]');
    
    // Should redirect to login
    await page.waitForURL('/login', { timeout: 5000 });
    await helpers.assert.assertURL(/\/login/);
  });

  test('should clear tokens on logout', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('access_token', 'mock-token');
    await helpers.data.setLocalStorageItem('refresh_token', 'mock-refresh-token');
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
    
    // Logout
    await page.click('[data-testid="user-menu"]');
    await page.click('[data-testid="logout-button"]');
    
    // Wait for redirect
    await page.waitForURL('/login');
    
    // Tokens should be cleared
    const accessToken = await helpers.data.getLocalStorageItem('access_token');
    const refreshToken = await helpers.data.getLocalStorageItem('refresh_token');
    
    expect(accessToken).toBeFalsy();
    expect(refreshToken).toBeFalsy();
  });

  test('should persist session across page reloads', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('access_token', 'mock-token');
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
    
    // Reload page
    await page.reload();
    
    // Should still be on dashboard (not redirected to login)
    await page.waitForLoadState('networkidle');
    await helpers.assert.assertURL(/\/dashboard/);
  });

  test('should handle expired token', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state with expired token
    await helpers.data.setLocalStorageItem('access_token', 'expired-token');
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Mock API call that returns 401
    await page.route('**/api/**', route => {
      route.fulfill({
        status: 401,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Token expired' })
      });
    });
    
    // Navigate to dashboard
    await page.goto('/dashboard');
    
    // Should redirect to login
    await page.waitForURL('/login', { timeout: 10000 });
    await helpers.assert.assertURL(/\/login/);
  });

  test('should refresh token automatically', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('access_token', 'old-token');
    await helpers.data.setLocalStorageItem('refresh_token', 'refresh-token');
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Mock token refresh
    await page.route('**/api/auth/refresh', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          access_token: 'new-token',
          refresh_token: 'new-refresh-token'
        })
      });
    });
    
    // Mock API call that triggers refresh
    let callCount = 0;
    await page.route('**/api/dashboard', route => {
      callCount++;
      if (callCount === 1) {
        // First call returns 401
        route.fulfill({
          status: 401,
          contentType: 'application/json',
          body: JSON.stringify({ error: 'Token expired' })
        });
      } else {
        // Second call succeeds with new token
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ data: 'success' })
        });
      }
    });
    
    // Navigate to dashboard
    await helpers.nav.goToDashboard();
    
    // Wait for token refresh
    await page.waitForTimeout(2000);
    
    // Should have new token
    const newToken = await helpers.data.getLocalStorageItem('access_token');
    expect(newToken).toBe('new-token');
  });

  test('should protect routes from unauthenticated access', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Try to access various protected routes
    const protectedRoutes = [
      '/dashboard',
      '/quran',
      '/ai-assistant',
      '/prayer-times',
      '/settings'
    ];
    
    for (const route of protectedRoutes) {
      await page.goto(route);
      
      // Should redirect to login
      await page.waitForURL('/login', { timeout: 5000 });
      await helpers.assert.assertURL(/\/login/);
    }
  });

  test('should allow access to public routes', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Public routes should be accessible without auth
    const publicRoutes = [
      '/login',
      '/onboarding'
    ];
    
    for (const route of publicRoutes) {
      await page.goto(route);
      
      // Should not redirect
      await page.waitForLoadState('networkidle');
      expect(page.url()).toContain(route);
    }
  });

  test('should handle network errors during login', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock network error
    await page.route('**/api/auth/login', route => {
      route.abort('failed');
    });
    
    // Navigate to login
    await page.goto('/login');
    
    // Fill in credentials
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Should show network error
    await helpers.wait.waitForVisible('[data-testid="login-error"]', 5000);
    
    // Error message should mention network issue
    const errorText = await page.locator('[data-testid="login-error"]').textContent();
    expect(errorText).toMatch(/network|connection|failed/i);
  });

  test('should disable submit button while logging in', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock slow login response
    await page.route('**/api/auth/login', async route => {
      await new Promise(resolve => setTimeout(resolve, 2000));
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          access_token: 'mock-token',
          refresh_token: 'mock-refresh-token'
        })
      });
    });
    
    // Navigate to login
    await page.goto('/login');
    
    // Fill in credentials
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Button should be disabled
    const isDisabled = await page.locator('button[type="submit"]').isDisabled();
    expect(isDisabled).toBe(true);
  });

  test('should show loading indicator during login', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Mock slow login response
    await page.route('**/api/auth/login', async route => {
      await new Promise(resolve => setTimeout(resolve, 2000));
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          access_token: 'mock-token',
          refresh_token: 'mock-refresh-token'
        })
      });
    });
    
    // Navigate to login
    await page.goto('/login');
    
    // Fill in credentials
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Should show loading indicator
    await helpers.wait.waitForVisible('[data-testid="loading-indicator"]', 1000);
  });

  test('should be responsive on mobile', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Navigate to login
    await page.goto('/login');
    
    // Form should be visible and usable
    await expect(page.locator('[data-testid="login-form"]')).toBeVisible();
    await expect(page.locator('input[name="email"]')).toBeVisible();
    await expect(page.locator('input[name="password"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
    
    // No horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });
});
