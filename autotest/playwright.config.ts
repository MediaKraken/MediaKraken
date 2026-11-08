import { defineConfig, devices } from '@playwright/test';

/**
 * Read environment variables from .env file.
 */
import 'dotenv/config';

import { baseOrigin } from './utils/env';

/**
 * e2e/auth-rate-limit.spec.ts hammers the shared client-IP auth budget
 * (login 10/60s, register 5/60s), so it is excluded from the browser
 * projects and only runs in the dedicated `rate-limit` project below.
 */
const RATE_LIMIT_SPEC = 'e2e/auth-rate-limit.spec.ts';

export default defineConfig({
  // All specs live in e2e/, integration/ and unit/ (see testMatch).
  testDir: '.',

  testMatch: ['**/*.spec.ts'],

  /** Global timeout per test (live-deployment network round trips). */
  timeout: 60_000,

  /** Run tests in files in parallel */
  fullyParallel: true,

  /** Fail the build on CI if you accidentally left test.only in the source code */
  forbidOnly: !!process.env.CI,

  /** Retry on CI only */
  retries: process.env.CI ? 2 : 0,

  /** Opt out of parallel tests on CI */
  workers: process.env.CI ? 1 : 4,

  /** Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: 'html',

  /** Shared settings for all projects */
  use: {
    /** Base URL to use in actions like `await page.goto('/')`. */
    baseURL: baseOrigin(),

    /** Collect trace when retrying the failed test */
    trace: 'on-first-retry',

    /** Take a screenshot on failure */
    screenshot: 'only-on-failure',

    /** Capture video on failure */
    video: 'retain-on-failure',
  },

  /**
   * Configure projects for major browsers. The browser projects must NOT
   * run in parallel with the rate-limit project (Playwright runs projects
   * concurrently): the burst would eat the shared-IP login/register budget
   * and flip the 303 expectations of auth-login/csrf/session-cookies into
   * 429s. Run them as separate commands (see README, CI section).
   */
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
      testIgnore: RATE_LIMIT_SPEC,
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
      testIgnore: RATE_LIMIT_SPEC,
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
      testIgnore: RATE_LIMIT_SPEC,
    },
    /** Optional: mobile emulation */
    {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
      testIgnore: RATE_LIMIT_SPEC,
    },
    /**
     * Rate-limit project: runs only e2e/auth-rate-limit.spec.ts, with a
     * single worker so its two bursts stay sequential against the shared
     * client-IP budget. Invoke explicitly:
     *
     *   npx playwright test --project=rate-limit
     */
    {
      name: 'rate-limit',
      use: { ...devices['Desktop Chrome'] },
      testMatch: RATE_LIMIT_SPEC,
      workers: 1,
      fullyParallel: false,
    },
  ],

  /** Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: 'test-results/',
});
