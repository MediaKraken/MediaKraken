/**
 * /user/search.
 *
 * DOCUMENTED QUIRK: the registered handler (main.rs) is the PUBLIC
 * `search_handler` from src/user/bp_search.rs — no auth, no permission
 * check, and the per-media-type fetchers are commented out, so the page
 * always renders "No results found" for everyone. The permission-gated
 * `user_search` handler exists in the same file but is NOT registered.
 *
 * These tests assert that current behavior so the suite goes red if the
 * wiring changes (in which case the authenticated expectations below need
 * revisiting).
 */
import { test, expect } from '../fixtures/auth';

test.describe('/user/search', () => {
  test('anonymous GET renders the results page (no auth required)', async ({ page }) => {
    // No login session exists in the default context, yet the page renders.
    await page.goto('/user/search');
    await expect(page).toHaveTitle('MediaKraken Search Results');
    await expect(page.getByRole('heading', { name: 'Search Results' })).toBeVisible();
    await expect(page.getByText('No results found')).toBeVisible();
  });

  test('a query parameter does not change the always-empty result', async ({ page }) => {
    await page.goto('/user/search?q=test');
    await expect(page).toHaveTitle('MediaKraken Search Results');
    await expect(page.getByText('No results found')).toBeVisible();
  });

  test('filter parameters are accepted and the result stays empty', async ({ request }) => {
    // The handler accepts a repeatable `filter` param; the "movie" branch is
    // commented out, so the result is still the empty state.
    const response = await request.get('/user/search?q=autotest&filter=movie');
    expect(response.status()).toBe(200);
    const body = await response.text();
    expect(body).toContain('No results found');
  });
});
