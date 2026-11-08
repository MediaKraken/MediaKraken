/**
 * Login flows through the real UI.
 *
 * Browser form submissions carry the site's own Origin header, so these
 * tests also exercise the CSRF middleware's happy path. The wrong-credential
 * and empty-form tests are always safe to run; the success test needs real
 * credentials (TEST_USERNAME / TEST_PASSWORD) and only asserts the redirect
 * — the fixture-verified specs (user-home, user-pages, ...) assert the
 * authenticated content.
 */
import { test, expect } from '../fixtures/auth';
import { loginCredentials } from '../utils/env';
import { loginViaPage, readFlashError } from '../utils/auth';

test.describe('login form (always runs)', () => {
  test('wrong credentials redirect back to the login page with a flash error', async ({
    page,
  }) => {
    await page.goto('/public/login');
    await page.getByPlaceholder('Username').fill('no_such_user_autotest');
    await page.getByPlaceholder('Password').fill('definitely-wrong-pw');
    await page.getByRole('button', { name: 'Login' }).click();

    // The 303 lands back on the login page; the flash cookie renders the
    // error banner (only present after the round trip).
    await expect(page).toHaveURL(/\/public\/login$/);
    expect(await readFlashError(page)).toContain('Unknown user or password incorrect!');
  });

  test('empty form is blocked by the browser (required inputs)', async ({ page }) => {
    await page.goto('/public/login');
    // The required attribute makes HTML5 constraint validation block empty
    // submits (toBeRequired is no longer in the type definitions).
    expect(await page.getByPlaceholder('Username').getAttribute('required')).not.toBeNull();
    expect(await page.getByPlaceholder('Password').getAttribute('required')).not.toBeNull();

    await page.getByRole('button', { name: 'Login' }).click();

    // HTML5 constraint validation prevents the submit: the page never
    // leaves /public/login and no flash error is rendered.
    await expect(page).toHaveURL(/\/public\/login$/);
    await expect(page.locator('[role="alert"]')).toHaveCount(0);
  });
});

test.describe('successful login (requires TEST_USERNAME / TEST_PASSWORD)', () => {
  test('valid credentials redirect to /user/home', async ({ page }) => {
    const credentials = loginCredentials();
    test.skip(!credentials, 'TEST_USERNAME / TEST_PASSWORD not set');
    if (!credentials) {
      return;
    }

    await loginViaPage(page, credentials.username, credentials.password, credentials.authyToken);
    // The handler always redirects to /user/home on success, even for
    // permission-less accounts (those then see the 401 page there).
    await expect(page).toHaveURL(/\/user\/home$/);
  });
});
