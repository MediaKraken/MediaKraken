/**
 * /user/search.
 *
 * The registered handler (main.rs) is the permission-gated `user_search`
 * from src/user/bp_search.rs: anonymous visitors get the 401 error page,
 * and any user with User::View gets the title-search form
 * (bss_user/bss_user_media_search.html). The handler takes no Query
 * extractor, so ?q= and filter parameters are accepted but ignored — the
 * rendered page is byte-identical regardless.
 */
import { test, expect } from '../fixtures/auth';

test.describe('/user/search', () => {
  test('anonymous GET gets the 401 error page', async ({ page }) => {
    // No login session exists in the default context; the handler's
    // permission check must reject with the 401 template.
    await page.goto('/user/search');
    await expect(page).toHaveTitle('MediaKraken – 401');
    await expect(
      page.getByRole('heading', { name: '401 – Must be a verified user' }),
    ).toBeVisible();
  });

  test('authenticated GET renders the title search form', async ({ authPage: page }) => {
    await page.goto('/user/search');
    await expect(page).toHaveTitle('MediaKraken Search');
    await expect(page.getByRole('heading', { name: 'Title Search' })).toBeVisible();
  });

  test('query and filter parameters are accepted but ignored', async ({ authRequest }) => {
    // No Query extractor in the handler: both requests render the exact
    // same page (the layout and template carry no per-request data).
    const plain = await authRequest.get('/user/search');
    expect(plain.status()).toBe(200);
    const withParams = await authRequest.get('/user/search?q=autotest&filter=movie');
    expect(withParams.status()).toBe(200);
    expect(await withParams.text()).toBe(await plain.text());
  });
});
