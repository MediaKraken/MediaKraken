/**
 * Generic HTTP behavior of the app router:
 *
 * - unregistered paths fall through to the app-level 404 fallback
 * - the fallback 404 is registered AFTER (inside of) the middleware stack, so
 *   its responses carry NO security headers — a documented gap asserted here
 * - wrong methods on existing routes yield 405 (once CSRF is satisfied)
 * - the root path "/" is not registered and 404s
 */
import { test, expect } from '../fixtures/auth';
import { baseOrigin } from '../utils/env';

test.describe('fallback 404', () => {
  test('unregistered path returns the 404 template page', async ({ request }) => {
    const response = await request.get('/definitely/not/a/real/route');
    expect(response.status()).toBe(404);
    expect(response.headers()['content-type']).toContain('text/html');
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 404</title>');
    expect(body).toContain('404 – File not found');
  });

  test('the root path is not registered and 404s', async ({ request }) => {
    const response = await request.get('/');
    expect(response.status()).toBe(404);
    expect(await response.text()).toContain('<title>MediaKraken – 404</title>');
  });

  test('DOCUMENTED GAP: fallback 404 lacks the security headers', async ({ request }) => {
    // The fallback handler is attached after the security-headers middleware
    // (app.fallback() in main.rs), so its responses skip the middleware stack
    // entirely. Asserting absence so the suite goes red if the gap is ever
    // fixed (and the assertion can be flipped to presence then).
    const response = await request.get('/definitely/not/a/real/route');
    expect(response.status()).toBe(404);
    expect(response.headers()['x-frame-options']).toBeUndefined();
    expect(response.headers()['x-content-type-options']).toBeUndefined();
    expect(response.headers()['cache-control']).toBeUndefined();
  });
});

test.describe('method handling', () => {
  test('PUT on a GET-only route returns 405 (with CSRF origin)', async ({ request }) => {
    const response = await request.put('/public/about', {
      headers: { origin: baseOrigin() },
    });
    expect(response.status()).toBe(405);
    // Axum advertises the allowed methods; GET routes also accept HEAD.
    expect(response.headers()['allow']).toContain('GET');
  });

  test('DELETE on a GET-only route returns 405 (with CSRF origin)', async ({ request }) => {
    const response = await request.delete('/public/about', {
      headers: { origin: baseOrigin() },
    });
    expect(response.status()).toBe(405);
  });

  test('POST on a GET-only route returns 405 (with CSRF origin)', async ({ request }) => {
    const response = await request.post('/public/about', {
      headers: { origin: baseOrigin() },
    });
    expect(response.status()).toBe(405);
  });
});

test.describe('error route shortcuts', () => {
  // The /error/* routes render the error templates with their matching
  // status codes.
  const errorRoutes: Array<[string, number]> = [
    ['/error/401', 401],
    ['/error/403', 403],
    ['/error/500', 500],
  ];
  for (const [path, status] of errorRoutes) {
    test(`GET ${path} returns ${status}`, async ({ request }) => {
      const response = await request.get(path);
      expect(response.status()).toBe(status);
      expect(response.headers()['content-type']).toContain('text/html');
    });
  }
});
