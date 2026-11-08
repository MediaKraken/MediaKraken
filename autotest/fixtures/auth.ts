/**
 * Auth fixtures for the MediaKraken test suite.
 *
 * - `authState` / `adminState` (worker-scoped): log in once per worker via a
 *   direct POST to /public/login, verify that the resulting session can load
 *   an authenticated page, and capture the cookie storage state. They return
 *   null when the corresponding env credentials are not set; every fixture
 *   derived from them then skips the test.
 *
 * - `authPage` / `adminPage` (test-scoped): a fresh page per test carrying
 *   the worker's storage state.
 *
 * - `authRequest` / `adminRequest` (test-scoped): an API request context
 *   carrying the worker's cookies, for request-level assertions.
 *
 * IMPORTANT — shared sessions: every page/context in one worker reuses the
 * SAME server-side session (one mm_session row). /public/logout destroys that
 * session, so the logout test performs its own dedicated login instead of
 * using these fixtures (see e2e/session-cookies.spec.ts).
 */
import { test as base, expect } from '@playwright/test';
import type { APIRequestContext, Page, PlaywrightWorkerArgs } from '@playwright/test';
// Playwright 1.65 no longer exports named `Playwright` / `StorageState` types,
// so derive them: the `playwright` fixture is the playwright-core module
// namespace, and the storage state is the return type of
// `APIRequestContext.storageState()`.
type Playwright = PlaywrightWorkerArgs['playwright'];
type StorageState = Awaited<ReturnType<APIRequestContext['storageState']>>;
import {
  adminCredentials,
  baseOrigin,
  ENV,
  loginCredentials,
  type Credentials,
} from '../utils/env';
import { loginViaRequest } from '../utils/auth';

/**
 * Worker-scoped fixtures: one login per worker, shared by every test in the
 * worker. 1.65's `extend<T, W>` requires worker fixtures in the second type
 * parameter, otherwise the `{ scope: 'worker' }` option does not type-check.
 */
type WorkerFixtures = {
  authState: StorageState | null;
  adminState: StorageState | null;
};

type TestFixtures = {
  authPage: Page;
  adminPage: Page;
  authRequest: APIRequestContext;
  adminRequest: APIRequestContext;
};

/**
 * Logs in with a dedicated request context and returns its storage state.
 * `verifyPath` proves the session actually authenticates (a 303 from the
 * login endpoint alone would not catch a permission-less account).
 */
async function buildState(
  playwright: Playwright,
  credentials: Credentials,
  who: string,
  verifyPath: string,
  permissionHint: string,
): Promise<StorageState> {
  const request = await playwright.request.newContext({ baseURL: baseOrigin() });
  try {
    const result = await loginViaRequest(
      request,
      credentials.username,
      credentials.password,
      credentials.authyToken,
    );
    if (!result.ok) {
      throw new Error(
        `${who} login failed: ${result.error ?? 'unknown error'}. ` +
          `Check the ${who === 'Admin' ? 'ADMIN_USERNAME / ADMIN_PASSWORD' : 'TEST_USERNAME / TEST_PASSWORD'} ` +
          'environment variables (and TEST_AUTHY_TOKEN / ADMIN_AUTHY_TOKEN if 2FA is enabled).',
      );
    }
    const check = await request.get(verifyPath);
    if (check.status() !== 200) {
      throw new Error(
        `${who} login redirected to /user/home but GET ${verifyPath} returned ` +
          `${check.status()}. The account exists but most likely is missing the ` +
          `${permissionHint} permission (user pages are permission-gated; ` +
          'newly registered accounts start with no permissions).',
      );
    }
    return await request.storageState();
  } finally {
    await request.dispose();
  }
}

export const test = base.extend<TestFixtures, WorkerFixtures>({
  authState: [
    async ({ playwright }, use) => {
      const credentials = loginCredentials();
      if (!credentials) {
        await use(null);
        return;
      }
      const state = await buildState(
        playwright,
        credentials,
        'Test user',
        '/user/home',
        'User::View',
      );
      await use(state);
    },
    { scope: 'worker' },
  ],

  adminState: [
    async ({ playwright }, use) => {
      const credentials = adminCredentials();
      if (!credentials) {
        await use(null);
        return;
      }
      // /admin/user/{page} is used instead of /admin/home for verification:
      // admin_home additionally calls an external IP lookup service, which
      // would make the fixture depend on third-party availability.
      const state = await buildState(
        playwright,
        credentials,
        'Admin',
        '/admin/user/1',
        'Admin::View',
      );
      await use(state);
    },
    { scope: 'worker' },
  ],

  authPage: async ({ browser, authState }, use) => {
    if (!authState) {
      // test.skip() throws, so authState is non-null past this point.
      test.skip(
        true,
        `${ENV.testUsername} / ${ENV.testPassword} not set; skipping authenticated test`,
      );
    }
    const page = await browser.newPage({ storageState: authState! });
    await use(page);
    await page.close();
  },

  adminPage: async ({ browser, adminState }, use) => {
    if (!adminState) {
      // test.skip() throws, so adminState is non-null past this point.
      test.skip(
        true,
        `${ENV.adminUsername} / ${ENV.adminPassword} not set; skipping admin test`,
      );
    }
    const page = await browser.newPage({ storageState: adminState! });
    await use(page);
    await page.close();
  },

  authRequest: async ({ playwright, authState }, use) => {
    if (!authState) {
      // test.skip() throws, so authState is non-null past this point.
      test.skip(
        true,
        `${ENV.testUsername} / ${ENV.testPassword} not set; skipping authenticated test`,
      );
    }
    const context = await playwright.request.newContext({
      baseURL: baseOrigin(),
      storageState: authState!,
    });
    await use(context);
    await context.dispose();
  },

  adminRequest: async ({ playwright, adminState }, use) => {
    if (!adminState) {
      // test.skip() throws, so adminState is non-null past this point.
      test.skip(
        true,
        `${ENV.adminUsername} / ${ENV.adminPassword} not set; skipping admin test`,
      );
    }
    const context = await playwright.request.newContext({
      baseURL: baseOrigin(),
      storageState: adminState!,
    });
    await use(context);
    await context.dispose();
  },
});

export { expect };
