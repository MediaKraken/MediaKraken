/**
 * Admin area access control.
 *
 * Every /admin/* handler requires the Admin::View permission. DOCUMENTED
 * BUG: the rejection renders the 403 template (bss_error_403.html) but
 * returns a 401 status — asserted here for both anonymous and signed-in
 * non-admin visitors.
 *
 * /admin/home is skipped for the admin success test: it additionally calls
 * myexternalip.com with an `.unwrap()`, which would couple the suite to a
 * third-party service (and the response can 500 when that call fails).
 */
import { test, expect } from '../fixtures/auth';

test.describe('admin pages without Admin::View', () => {
  test('anonymous access gets a 401 status with the 403 template', async ({ request }) => {
    const response = await request.get('/admin/user/1');
    expect(response.status()).toBe(401);
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 403</title>');
    expect(body).toContain('403 – Invalid security profile');
  });

  test('signed-in non-admin gets the same 401-status/403-template response', async ({
    authRequest,
  }) => {
    const response = await authRequest.get('/admin/user/1');
    expect(response.status()).toBe(401);
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 403</title>');
    expect(body).toContain('403 – Invalid security profile');
  });
});

test.describe('admin pages with Admin::View (requires ADMIN_USERNAME / ADMIN_PASSWORD)', () => {
  test('the user management page renders', async ({ adminPage: page }) => {
    await page.goto('/admin/user/1');
    await expect(page).toHaveTitle('MediaKraken Admin User');
  });
});
