/**
 * Error pages: the /error/* shortcut routes plus the error pages the app
 * produces in the real world.
 *
 * Template strings come from templates/bss_error/bss_error_{401,403,404,500}.html.
 */
import { test, expect } from '../fixtures/auth';

test.describe('/error shortcuts', () => {
  test('401 renders the verified-user error page', async ({ page }) => {
    await page.goto('/error/401');
    await expect(page).toHaveTitle('MediaKraken – 401');
    await expect(
      page.getByRole('heading', { name: '401 – Must be a verified user' }),
    ).toBeVisible();
  });

  test('403 renders the admin error page', async ({ page }) => {
    await page.goto('/error/403');
    await expect(page).toHaveTitle('MediaKraken – 403');
    await expect(
      page.getByRole('heading', { name: '403 – Invalid security profile' }),
    ).toBeVisible();
  });

  test('500 renders the application failure page', async ({ page }) => {
    await page.goto('/error/500');
    await expect(page).toHaveTitle('MediaKraken – 500');
    await expect(page.getByRole('heading', { name: '500 – Application failure' })).toBeVisible();
  });
});

test.describe('real error surfaces', () => {
  test('anonymous /user/home is a 401 with the 401 template', async ({ request }) => {
    const response = await request.get('/user/home');
    expect(response.status()).toBe(401);
    expect(response.headers()['content-type']).toContain('text/html');
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 401</title>');
    expect(body).toContain('401 – Must be a verified user');
  });

  test('DOCUMENTED BUG: /admin/* answers a 401 status with the 403 template', async ({
    request,
  }) => {
    // Every admin handler (src/admin/bp_*.rs) renders
    // bss_error/bss_error_403.html but returns StatusCode::UNAUTHORIZED.
    // Asserting both sides of the mismatch so it stays visible in the suite.
    const response = await request.get('/admin/home');
    expect(response.status()).toBe(401);
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 403</title>');
    expect(body).toContain('403 – Invalid security profile');
  });

  test('unregistered path renders the fallback 404 page', async ({ request }) => {
    const response = await request.get('/definitely/not/a/real/route');
    expect(response.status()).toBe(404);
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 404</title>');
    expect(body).toContain('404 – File not found');
  });
});
