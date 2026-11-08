/**
 * Navigation behavior.
 *
 * The authenticated top navbar (templates/partials/topnavbar.html) hides its
 * link row and the center search form below the `sm`/`md` Tailwind
 * breakpoints, so navbar-interaction tests are skipped on the Mobile Chrome
 * project (a helper is inlined; the project name is fixed in
 * playwright.config.ts).
 */
import { test, expect } from '../fixtures/auth';

const isMobile = (): boolean => test.info().project.name === 'Mobile Chrome';

test.describe('public page cross-links', () => {
  test('login page links to register and forgot-password', async ({ page }) => {
    await page.goto('/public/login');
    await page.getByRole('link', { name: 'Create an Account!' }).click();
    await expect(page).toHaveURL(/\/public\/register$/);

    await page.goto('/public/register');
    await page.getByRole('link', { name: 'Forgot Password?' }).click();
    await expect(page).toHaveURL(/\/public\/forgot_password$/);
  });
});

test.describe('authenticated navbar', () => {
  test('shows the main nav links and signed-in badge', async ({ authPage: page }) => {
    test.skip(isMobile(), 'navbar links are hidden below the sm breakpoint');
    await page.goto('/user/home');
    await expect(page.getByRole('link', { name: 'Home' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Sync' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Profile' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Admin' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Metadata' })).toBeVisible();
    await expect(page.getByText('Signed in')).toBeVisible();
  });

  test('nav link Home navigates to the dashboard', async ({ authPage: page }) => {
    test.skip(isMobile(), 'navbar links are hidden below the sm breakpoint');
    await page.goto('/user/queue');
    await page.getByRole('link', { name: 'Home' }).click();
    await expect(page).toHaveURL(/\/user\/home$/);
    await expect(page.getByRole('heading', { name: 'Home' })).toBeVisible();
  });

  test('navbar search submits to /user/search and shows the results page', async ({
    authPage: page,
  }) => {
    test.skip(isMobile(), 'navbar search form is hidden below the md breakpoint');
    await page.goto('/user/home');
    await page.getByPlaceholder('Search media...').fill('test query');
    await page.getByPlaceholder('Search media...').press('Enter');
    // The query string is percent-encoded in the location bar.
    await expect(page).toHaveURL(/\/user\/search\?q=test%20query/);
    await expect(page).toHaveTitle('MediaKraken Search Results');
    await expect(page.getByRole('heading', { name: 'Search Results' })).toBeVisible();
  });

  test('theme toggle button is available', async ({ authPage: page }) => {
    await page.goto('/user/home');
    const toggle = page.getByRole('button', { name: 'Toggle dark mode' });
    await expect(toggle).toBeVisible();
    await toggle.click();
    // The toggle persists the choice to localStorage (base_user.html).
    await expect
      .poll(() => page.evaluate(() => window.localStorage.getItem('theme')))
      .not.toBeNull();
  });
});
