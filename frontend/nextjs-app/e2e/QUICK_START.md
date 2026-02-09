# E2E Tests Quick Start Guide

## Installation

```bash
cd frontend/nextjs-app
npm install
```

## First Time Setup

Install Playwright browsers:

```bash
npx playwright install
```

Or install with system dependencies:

```bash
npx playwright install --with-deps
```

## Running Tests

### Basic Commands

```bash
# Run all tests
npm run test:e2e

# Run with UI (recommended for development)
npm run test:e2e:ui

# Run in headed mode (see browser)
npm run test:e2e:headed

# Run in debug mode
npm run test:e2e:debug
```

### Browser-Specific

```bash
# Chromium only
npm run test:e2e:chromium

# Firefox only
npm run test:e2e:firefox

# WebKit/Safari only
npm run test:e2e:webkit

# Mobile browsers
npm run test:e2e:mobile
```

### Specific Test Files

```bash
# Run onboarding tests
npx playwright test e2e/01-onboarding.spec.ts

# Run dashboard tests
npx playwright test e2e/02-dashboard.spec.ts

# Run Quran reading tests
npx playwright test e2e/03-quran-reading.spec.ts

# Run AI assistant tests
npx playwright test e2e/04-ai-assistant.spec.ts

# Run offline tests
npx playwright test e2e/05-offline-functionality.spec.ts

# Run authentication tests
npx playwright test e2e/06-authentication.spec.ts
```

### Specific Tests

```bash
# Run tests matching a pattern
npx playwright test -g "should login"

# Run a single test
npx playwright test e2e/06-authentication.spec.ts -g "should login with valid credentials"
```

## Viewing Results

### HTML Report

After tests complete:

```bash
npm run test:e2e:report
```

This opens an interactive HTML report in your browser.

### Test Artifacts

Failed tests generate:
- Screenshots: `test-results/*/test-failed-*.png`
- Videos: `test-results/*/video.webm`
- Traces: `test-results/*/trace.zip`

View a trace:

```bash
npx playwright show-trace test-results/path-to-trace.zip
```

## Development Workflow

### 1. Write Test

Create or edit a test file in `e2e/`:

```typescript
test('should do something', async ({ page }) => {
  const helpers = createHelpers(page);
  await helpers.nav.goToDashboard();
  await helpers.assert.assertVisible('[data-testid="element"]');
});
```

### 2. Run in UI Mode

```bash
npm run test:e2e:ui
```

This gives you:
- Interactive test runner
- Watch mode
- Time travel debugging
- Step-by-step execution

### 3. Debug Failures

```bash
npm run test:e2e:debug
```

Or use UI mode's built-in debugger.

### 4. Run Full Suite

```bash
npm run test:e2e
```

## Common Issues

### Dev Server Not Starting

Make sure port 3000 is available:

```bash
# Kill process on port 3000
npx kill-port 3000

# Or change port in playwright.config.ts
```

### Tests Timing Out

Increase timeout in test:

```typescript
test('slow test', async ({ page }) => {
  test.setTimeout(120000); // 2 minutes
  // test code
});
```

### Element Not Found

Add proper waits:

```typescript
await helpers.wait.waitForVisible('[data-testid="element"]');
```

### Flaky Tests

Use network idle:

```typescript
await page.waitForLoadState('networkidle');
```

## Tips

### Speed Up Tests

```bash
# Run fewer browsers
npx playwright test --project=chromium

# Run specific tests
npx playwright test e2e/01-onboarding.spec.ts
```

### Watch Mode

```bash
# UI mode has built-in watch
npm run test:e2e:ui
```

### Parallel Execution

Tests run in parallel by default. To run serially:

```bash
npx playwright test --workers=1
```

### Update Snapshots

If using visual regression:

```bash
npx playwright test --update-snapshots
```

## CI/CD

### GitHub Actions Example

```yaml
- name: Install dependencies
  run: npm ci

- name: Install Playwright Browsers
  run: npx playwright install --with-deps

- name: Run E2E tests
  run: npm run test:e2e

- name: Upload test results
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: playwright-report
    path: playwright-report/
```

### Environment Variables

```bash
# Custom base URL
BASE_URL=https://staging.example.com npm run test:e2e

# CI mode (more retries)
CI=true npm run test:e2e
```

## Next Steps

1. Read the full [README](./README.md)
2. Check [test helpers](./helpers/test-helpers.ts)
3. Review existing test files
4. Write your first test
5. Run in UI mode for feedback

## Resources

- [Playwright Docs](https://playwright.dev/)
- [Test Helpers](./helpers/test-helpers.ts)
- [Full README](./README.md)
- [Implementation Summary](../E2E_TESTS_IMPLEMENTATION.md)

## Getting Help

1. Check this guide
2. Read the README
3. Review Playwright docs
4. Ask the team

Happy testing! 🎭
