import { defineConfig, devices } from '@playwright/test';

/**
 * Read environment variables from .env file.
 */
import 'dotenv/config';

export default defineConfig({
  testDir: './tests',

  /** Run tests in files in parallel */
  fullyParallel: true,

  /** Fail the build on CI if you accidentally left test.only in the source code */
  forbidOnly: !!process.env.CI,

  /** Retry on CI only */
  retries: process.env.CI ? 2 : 0,

  /** Opt out of parallel tests on CI */
  workers: process.env.CI ? 1 : undefined,

  /** Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: 'html',

  /** Shared settings for all projects */
  use: {
    /** Base URL to use in actions like `await page.goto('/')`. */
    baseURL: process.env.BASE_URL ?? 'http://localhost:8080',

    /** Collect trace when retrying the failed test */
    trace: 'on-first-retry',

    /** Take a screenshot on failure */
    screenshot: 'only-on-failure',

    /** Capture video on failure */
    video: 'retain-on-failure',
  },

  /** Configure projects for major browsers */
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
    /** Optional: mobile emulation */
    {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
    },
  ],

  /** Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: 'test-results/',
});
