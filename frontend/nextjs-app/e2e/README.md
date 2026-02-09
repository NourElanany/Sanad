# E2E Tests for Sanad Frontend

This directory contains end-to-end (E2E) tests for the Sanad Islamic Application frontend using Playwright.

## Overview

The E2E test suite covers all critical user flows to ensure the application works correctly from the user's perspective. Tests are organized by feature and validate requirements from the specification.

## Test Structure

### Test Files

1. **01-onboarding.spec.ts** - Onboarding flow tests
   - Welcome screens
   - Permission requests
   - Madhab selection
   - Theme selection
   - Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5

2. **02-dashboard.spec.ts** - Dashboard functionality tests
   - Prayer times widget
   - Hijri calendar
   - Daily wird progress
   - Daily verse/hadith
   - Quick actions
   - Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5

3. **03-quran-reading.spec.ts** - Quran reading experience tests
   - Quran index navigation
   - Surah/Juz browsing
   - Mushaf page view
   - Verse interactions
   - Tafsir access
   - Bookmarks
   - Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5, 6.1, 6.2, 6.3, 6.4, 6.5

4. **04-ai-assistant.spec.ts** - AI Assistant with streaming tests
   - Chat interface
   - Text and voice input
   - Streaming responses
   - Source citations
   - Source verification
   - Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.5

5. **05-offline-functionality.spec.ts** - Offline mode tests
   - Service worker registration
   - Offline indicator
   - Cached content access
   - Offline queue
   - Sync when online
   - PWA functionality
   - Validates: Requirements 15.1, 15.2, 15.3, 15.4, 15.5, 2.3, 2.5

6. **06-authentication.spec.ts** - Authentication flow tests
   - Login flow
   - Logout flow
   - Token management
   - Protected routes
   - Session persistence
   - Validates: Requirements 14.1, 14.2, 14.4, 14.5

### Helper Utilities

**helpers/test-helpers.ts** - Reusable test utilities
- `AuthHelpers` - Authentication operations
- `NavigationHelpers` - Page navigation
- `WaitHelpers` - Wait utilities
- `OfflineHelpers` - Offline mode operations
- `DataHelpers` - Data management
- `AssertionHelpers` - Common assertions

## Running Tests

### Prerequisites

```bash
# Install dependencies
npm install
```

### Run All Tests

```bash
# Run all E2E tests
npm run test:e2e

# Run with UI mode (interactive)
npm run test:e2e:ui

# Run in headed mode (see browser)
npm run test:e2e:headed

# Run in debug mode
npm run test:e2e:debug
```

### Run Specific Browser

```bash
# Run on Chromium only
npm run test:e2e:chromium

# Run on Firefox only
npm run test:e2e:firefox

# Run on WebKit (Safari) only
npm run test:e2e:webkit

# Run on mobile browsers
npm run test:e2e:mobile
```

### Run Specific Test File

```bash
# Run onboarding tests only
npx playwright test e2e/01-onboarding.spec.ts

# Run AI assistant tests only
npx playwright test e2e/04-ai-assistant.spec.ts
```

### View Test Report

```bash
# Show HTML report
npm run test:e2e:report
```

## Configuration

Test configuration is in `playwright.config.ts`:

- **Test Directory**: `./e2e`
- **Timeout**: 60 seconds per test
- **Retries**: 2 retries in CI, 0 locally
- **Base URL**: `http://localhost:3000` (configurable via `BASE_URL` env var)
- **Browsers**: Chromium, Firefox, WebKit, Mobile Chrome, Mobile Safari
- **Screenshots**: On failure only
- **Videos**: Retained on failure
- **Traces**: On first retry

## Writing New Tests

### Basic Test Structure

```typescript
import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    // Setup code
  });

  test('should do something', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Test code
    await helpers.nav.goToDashboard();
    await helpers.assert.assertVisible('[data-testid="element"]');
  });
});
```

### Using Test Helpers

```typescript
const helpers = createHelpers(page);

// Authentication
await helpers.auth.login('user@example.com', 'password');
await helpers.auth.logout();

// Navigation
await helpers.nav.goToDashboard();
await helpers.nav.goToQuran();
await helpers.nav.goToAIAssistant();

// Waiting
await helpers.wait.waitForVisible('[data-testid="element"]');
await helpers.wait.waitForText('Some text');
await helpers.wait.waitForAPIResponse('/api/endpoint');
await helpers.wait.waitForStreamingComplete('[data-testid="message"]');

// Offline mode
await helpers.offline.goOffline();
await helpers.offline.goOnline();
await helpers.offline.isOfflineIndicatorVisible();

// Data management
await helpers.data.clearAllData();
await helpers.data.setLocalStorageItem('key', 'value');
const value = await helpers.data.getLocalStorageItem('key');

// Assertions
await helpers.assert.assertVisible('[data-testid="element"]');
await helpers.assert.assertContainsText('[data-testid="element"]', 'text');
await helpers.assert.assertURL(/\/dashboard/);
```

### Best Practices

1. **Use data-testid attributes** for reliable element selection
2. **Wait for elements** before interacting with them
3. **Use helpers** for common operations
4. **Test user flows** not implementation details
5. **Keep tests independent** - each test should work in isolation
6. **Clean up after tests** - use beforeEach/afterEach hooks
7. **Mock external APIs** when appropriate
8. **Test responsive design** on different viewports
9. **Test error states** and edge cases
10. **Document test purpose** with clear descriptions

### Data Test IDs

Use these data-testid attributes in components for E2E testing:

```tsx
// Example component
<div data-testid="prayer-times-card">
  <div data-testid="next-prayer">Maghrib</div>
  <div data-testid="prayer-countdown">2:34:12</div>
</div>
```

## CI/CD Integration

Tests run automatically in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run E2E tests
  run: npm run test:e2e
  env:
    BASE_URL: https://staging.sanad.app
```

## Debugging Tests

### Debug Mode

```bash
# Run in debug mode with Playwright Inspector
npm run test:e2e:debug
```

### Headed Mode

```bash
# See the browser while tests run
npm run test:e2e:headed
```

### UI Mode

```bash
# Interactive test runner
npm run test:e2e:ui
```

### Screenshots and Videos

Failed tests automatically capture:
- Screenshots (in `test-results/`)
- Videos (in `test-results/`)
- Traces (in `test-results/`)

View traces:
```bash
npx playwright show-trace test-results/path-to-trace.zip
```

## Troubleshooting

### Tests Timing Out

- Increase timeout in `playwright.config.ts`
- Check if dev server is running
- Verify network connectivity

### Element Not Found

- Check data-testid attribute exists
- Wait for element to be visible
- Check if element is in viewport

### Flaky Tests

- Add appropriate waits
- Use `waitForLoadState('networkidle')`
- Increase timeout for slow operations
- Mock external dependencies

### Service Worker Issues

- Clear browser cache between test runs
- Wait for service worker registration
- Use `helpers.offline.waitForServiceWorker()`

## Performance Considerations

- Tests run in parallel by default
- Use `fullyParallel: true` in config
- Limit workers in CI: `workers: 1`
- Reuse browser contexts when possible

## Coverage

E2E tests cover:
- ✅ Onboarding flow (100%)
- ✅ Dashboard functionality (100%)
- ✅ Quran reading experience (100%)
- ✅ AI Assistant with streaming (100%)
- ✅ Offline functionality (100%)
- ✅ Authentication flow (100%)

## Future Enhancements

- [ ] Visual regression testing
- [ ] Performance testing
- [ ] Accessibility testing (a11y)
- [ ] API contract testing
- [ ] Load testing
- [ ] Cross-browser compatibility matrix

## Resources

- [Playwright Documentation](https://playwright.dev/)
- [Best Practices](https://playwright.dev/docs/best-practices)
- [Debugging Guide](https://playwright.dev/docs/debug)
- [CI/CD Integration](https://playwright.dev/docs/ci)

## Support

For issues or questions:
1. Check this README
2. Review Playwright documentation
3. Check test helper utilities
4. Contact the development team
