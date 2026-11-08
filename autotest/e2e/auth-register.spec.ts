/**
 * Registration.
 *
 * The validation tests never insert a user (validation fails first) and are
 * safe to always run. The success/duplicate tests create REAL user rows on
 * the target deployment, so they only run when MK_TEST_ALLOW_REGISTER is
 * truthy.
 *
 * Note on the (currently inert) register rate limiter: this file performs at
 * most 3 register POSTs per run, and the rate-limit spec performs 4; both
 * stay under the configured 5/60s limit per file even if the limiter were
 * ever activated.
 */
import type { Page, TestInfo } from '@playwright/test';
import { test, expect } from '../fixtures/auth';
import { allowRegister, randomPassword, randomUsername } from '../utils/env';
import { readFlashError, submitRegisterForm } from '../utils/auth';

/**
 * Unique username per test. MK_TEST_RUN_ID makes randomUsername()
 * deterministic within a run, which would collide across the two parallel
 * success tests; the per-test id keeps them unique either way.
 */
function usernameFor(info: TestInfo): string {
  // testId is stable per test within a run (testIndex was removed).
  return `${randomUsername()}_${info.testId}`.slice(0, 64);
}

async function expectValidationError(page: Page, message: string): Promise<void> {
  await expect(page).toHaveURL(/\/public\/register$/);
  expect(await readFlashError(page)).toContain(message);
}

test.describe('registration validation (always runs)', () => {
  test('username that is too short is rejected', async ({ page }) => {
    await submitRegisterForm(page, 'ab', 'valid-password-123');
    await expectValidationError(page, 'Username must be between 3 and 64 characters.');
  });

  test('username with invalid characters is rejected', async ({ page }) => {
    // Space is outside the allowed [A-Za-z0-9_-.@] set.
    await submitRegisterForm(page, 'bad user name', 'valid-password-123');
    await expectValidationError(page, 'Username contains invalid characters.');
  });

  test('password that is too short is rejected', async ({ page }) => {
    // Username is valid so validation reaches the password length check.
    await submitRegisterForm(page, 'mktest_validuser', 'short');
    await expectValidationError(page, 'Password must be between 10 and 128 characters.');
  });
});

test.describe('registration success (requires MK_TEST_ALLOW_REGISTER)', () => {
  test('a valid new account redirects to the login page', async ({ page }) => {
    test.skip(
      !allowRegister(),
      'MK_TEST_ALLOW_REGISTER not set; registration inserts a real user row',
    );
    if (!allowRegister()) {
      return;
    }

    const username = usernameFor(test.info());
    await submitRegisterForm(page, username, randomPassword(username));

    // Success redirects to the login page with no flash error.
    await expect(page).toHaveURL(/\/public\/login$/);
    await expect(page.locator('[role="alert"]')).toHaveCount(0);
  });

  test('registering the same username twice reports an existing user', async ({ page }) => {
    test.skip(
      !allowRegister(),
      'MK_TEST_ALLOW_REGISTER not set; registration inserts a real user row',
    );
    if (!allowRegister()) {
      return;
    }

    const username = usernameFor(test.info());
    const password = randomPassword(username);

    // First attempt creates the account.
    await submitRegisterForm(page, username, password);
    await expect(page).toHaveURL(/\/public\/login$/);

    // Second attempt with the same username hits the existence check.
    await submitRegisterForm(page, username, password);
    await expectValidationError(page, 'User already exists!');
  });
});
