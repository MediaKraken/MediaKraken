/**
 * Public (unauthenticated) pages: login, register, forgot password, about.
 *
 * Selectors come from templates/bss_public/*.html:
 * - placeholders "Username", "Password", "2FA Code (if enabled)"
 * - buttons "Login", "Register Account", "Reset Password"
 * - footer "© Quinn D Granfor 2026" (included on all public pages)
 */
import { test, expect } from '../fixtures/auth';

test.describe('login page', () => {
  test('renders the login form', async ({ page }) => {
    await page.goto('/public/login');
    await expect(page).toHaveTitle('MediaKraken – Login');
    await expect(page.getByRole('heading', { name: 'Welcome Back!' })).toBeVisible();
    await expect(page.getByPlaceholder('Username')).toBeVisible();
    await expect(page.getByPlaceholder('Password')).toBeVisible();
    await expect(page.getByPlaceholder('2FA Code (if enabled)')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Login' })).toBeVisible();
  });

  test('shows the footer', async ({ page }) => {
    await page.goto('/public/login');
    await expect(page.getByText('© Quinn D Granfor 2026')).toBeVisible();
  });
});

test.describe('register page', () => {
  test('renders the registration form', async ({ page }) => {
    await page.goto('/public/register');
    await expect(page).toHaveTitle('MediaKraken – Register');
    await expect(page.getByRole('heading', { name: 'Create an Account!' })).toBeVisible();
    await expect(page.getByPlaceholder('Username')).toBeVisible();
    await expect(page.getByPlaceholder('Password')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Register Account' })).toBeVisible();
  });
});

test.describe('forgot password page', () => {
  test('renders the reset form', async ({ page }) => {
    await page.goto('/public/forgot_password');
    await expect(page).toHaveTitle('MediaKraken – Forgotten Password');
    await expect(page.getByRole('heading', { name: 'Forgot Your Password?' })).toBeVisible();
    await expect(page.getByPlaceholder('Enter email address…')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Reset Password' })).toBeVisible();
  });

  test('POST answers that resets are not yet enabled', async ({ page }) => {
    // Verified live: the handler returns a 200 with a plain-text notice
    // instead of performing any work.
    await page.goto('/public/forgot_password');
    await page.getByPlaceholder('Enter email address…').fill('test@example.com');
    await page.getByRole('button', { name: 'Reset Password' }).click();
    // Page-level toHaveText is no longer in the type definitions.
    await expect(page.locator('body')).toContainText(/Password reset requests are not yet enabled\./);
  });
});

test.describe('about page', () => {
  test('renders the about content', async ({ page }) => {
    await page.goto('/public/about');
    // The template writes the title with an &ndash; entity; the DOM resolves
    // it to the en dash.
    await expect(page).toHaveTitle('MediaKraken – About');
    await expect(page.getByRole('heading', { name: 'About MediaKraken' })).toBeVisible();
  });
});
