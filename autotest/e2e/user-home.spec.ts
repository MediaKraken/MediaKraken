/**
 * The authenticated dashboard (/user/home) — the landing page after login
 * and the target of every "Home" nav link.
 */
import { test, expect } from '../fixtures/auth';

test.describe('user dashboard', () => {
  test('renders the dashboard for a signed-in user', async ({ authPage: page }) => {
    await page.goto('/user/home');
    await expect(page).toHaveTitle('MediaKraken');
    await expect(page.getByRole('heading', { name: 'Home', level: 1 })).toBeVisible();
    await expect(
      page.getByRole('heading', { name: 'Browse your library', level: 2 }),
    ).toBeVisible();
  });
});
