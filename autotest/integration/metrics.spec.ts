/**
 * /metrics — Prometheus exposition endpoint (axum-prometheus layer).
 *
 * The counter label values are created lazily on first observation, so the
 * test makes an instrumented request first and then checks that the
 * /health_check series exists.
 */
import { test, expect } from '../fixtures/auth';

test.describe('/metrics', () => {
  test('returns Prometheus text format with request counters', async ({ request }) => {
    // Ensure at least one instrumented request exists for this window.
    const probe = await request.get('/health_check');
    expect(probe.status()).toBe(200);

    const response = await request.get('/metrics');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/plain');

    const body = await response.text();
    expect(body).toContain('# TYPE');
    expect(body).toContain('axum_http_requests_total');
    expect(body).toContain('endpoint="/health_check"');
  });

  test('response carries the standard security headers', async ({ request }) => {
    const response = await request.get('/metrics');
    expect(response.status()).toBe(200);
    expect(response.headers()['x-content-type-options']).toBe('nosniff');
    expect(response.headers()['x-frame-options']).toBe('DENY');
    expect(response.headers()['cache-control']).toBe('no-store, no-cache, must-revalidate');
  });
});
