/**
 * /health_check — the probe endpoint used by the load balancer / k8s.
 *
 * Verified against the live deployment:
 * - GET  -> 200, empty body, text/html
 * - HEAD -> 200
 * - trailing slash -> 308 Permanent Redirect to the canonical path
 * - security headers present (route is registered under the middleware stack)
 */
import { test, expect } from '../fixtures/auth';

test.describe('/health_check', () => {
  test('GET returns 200 with an empty body', async ({ request }) => {
    const response = await request.get('/health_check');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/html');
    expect((await response.body()).length).toBe(0);
  });

  test('HEAD returns 200', async ({ request }) => {
    const response = await request.head('/health_check');
    expect(response.status()).toBe(200);
  });

  test('trailing slash is a 308 redirect to the canonical path', async ({ request }) => {
    const response = await request.get('/health_check/', { maxRedirects: 0 });
    expect(response.status()).toBe(308);
    expect(response.headers()['location']).toBe('/health_check');
  });

  test('response carries the standard security headers', async ({ request }) => {
    const response = await request.get('/health_check');
    expect(response.headers()['x-content-type-options']).toBe('nosniff');
    expect(response.headers()['x-frame-options']).toBe('DENY');
    expect(response.headers()['x-xss-protection']).toBe('1; mode=block');
    expect(response.headers()['referrer-policy']).toBe('strict-origin-when-cross-origin');
    expect(response.headers()['cache-control']).toBe('no-store, no-cache, must-revalidate');
  });
});
