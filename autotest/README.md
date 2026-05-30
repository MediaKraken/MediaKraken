# MediaKraken Autotest

Playwright-based end-to-end and integration tests for MediaKraken.

## Quick Start

### Prerequisites

- **Node.js** ≥ 18 (LTS recommended)
- **npm** ≥ 9
- A running MediaKraken backend (default: `http://localhost:8080`)

### Installation

```bash
# 1. Navigate to the autotest directory
cd autotest

# 2. Install npm dependencies
npm install

# 3. Install Playwright browsers (Chromium, Firefox, WebKit)
npx playwright install

# Optional: install system dependencies required by browsers
npx playwright install-deps
```

### Running Tests

```bash
# Run all tests
npx playwright test

# Run a specific test file
npx playwright test tests/example.spec.ts

# Run tests in headed mode (browser window visible)
npx playwright test --headed

# Run tests in debug mode
npx playwright test --debug

# Run tests for a specific project/browser
npx playwright test --project=chromium

# Generate a trace for a failed test
npx playwright test --trace on
```

### Configuration

Edit `playwright.config.ts` to customize:

- `baseURL` — MediaKraken server URL
- `projects` — browser configurations
- `timeout` — global and per-test timeouts
- `use` — viewport, navigation wait, etc.

## Project Structure

```
autotest/
├── tests/           # Shared / base test helpers and page objects
├── e2e/             # End-to-end tests (full user flows)
├── integration/     # API and service integration tests
├── unit/            # Unit tests for shared utilities
├── fixtures/        # Playwright fixtures (auth, data seeds, etc.)
└── utils/           # Helper functions and utilities
```

## Writing Tests

1. Place E2E tests in `e2e/`, integration tests in `integration/`.
2. Use `tests/` for shared page-object models and base fixtures.
3. Put reusable helpers in `utils/`.
4. Define custom fixtures in `fixtures/`.

Example:

```ts
import { test, expect } from '@playwright/test';

test('home page loads', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/MediaKraken/);
});
```

## CI / CD

Integrate with your CI pipeline by running:

```bash
npm ci          # install from package-lock.json
npx playwright install --with-deps chromium  # headless browser
npx playwright test --reporter=github          # GitHub Actions-friendly output
```

## Tips

- Use `--retries` for flaky tests: `npx playwright test --retries 2`
- Use `--shard` to parallelize across machines or workers.
- Use `trace: on` to capture execution traces for debugging.
