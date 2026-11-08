/**
 * /static file serving (ServeDir).
 *
 * Verified against the live deployment. Note the interaction between two
 * layers:
 * - the nested /static service sets `cache-control: public, max-age=31536000,
 *   immutable` (main.rs nest_service),
 * - the outer security-headers middleware then OVERWRITES cache-control with
 *   `no-store, no-cache, must-revalidate`, which is what clients actually see.
 */
import { test, expect } from '../fixtures/auth';

test.describe('/static', () => {
  test('serves the main stylesheet as text/css', async ({ request }) => {
    const response = await request.get('/static/css/base_webapp_9.css');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/css');
    expect((await response.text()).length).toBeGreaterThan(0);
  });

  test('serves Alpine.js', async ({ request }) => {
    const response = await request.get('/static/js/alpinejs_3.15.3.min.js');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('javascript');
  });

  test('serves the app script', async ({ request }) => {
    const response = await request.get('/static/js/base_webapp.js');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('javascript');
  });

  test('serves robots.txt disallowing crawlers', async ({ request }) => {
    const response = await request.get('/static/robots.txt');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/plain');
    const body = await response.text();
    expect(body).toContain('User-agent: *');
    expect(body).toContain('Disallow: /');
  });

  test('serves the site logo image', async ({ request }) => {
    const response = await request.get('/static/image/K3.png');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('image/png');
  });

  test('security headers override the static immutable cache-control', async ({
    request,
  }) => {
    const response = await request.get('/static/css/base_webapp_9.css');
    // The outer security middleware wins over the nested service header.
    expect(response.headers()['cache-control']).toBe('no-store, no-cache, must-revalidate');
    expect(response.headers()['x-frame-options']).toBe('DENY');
  });

  test('directory path without index is a 404', async ({ request }) => {
    const response = await request.get('/static/css/');
    expect(response.status()).toBe(404);
  });

  test('missing file is a 404 but keeps the security headers', async ({ request }) => {
    // A 404 from the /static service itself passes through the middleware
    // stack (unlike the app-level fallback 404, see e2e/error-pages.spec.ts).
    const response = await request.get('/static/missing.css');
    expect(response.status()).toBe(404);
    expect(response.headers()['x-content-type-options']).toBe('nosniff');
    expect(response.headers()['x-frame-options']).toBe('DENY');
    expect(response.headers()['cache-control']).toBe('no-store, no-cache, must-revalidate');
  });
});
