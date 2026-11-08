/**
 * Security headers (security_headers middleware in main.rs).
 *
 * The middleware inserts a fixed header set into every response and strips
 * the app's Server header. It is the OUTERMOST middleware, so even
 * middleware rejections (the CSRF 403) carry the headers, and the 404
 * fallback — merged as a sub-router with its own security_headers layer —
 * carries them too. (Both of those were documented gaps before the
 * middleware was moved outside require_same_origin.)
 */
import type { APIResponse } from '@playwright/test';
import { test, expect } from '../fixtures/auth';
import { baseOrigin } from '../utils/env';

const SECURITY_HEADERS: Record<string, string> = {
  'x-content-type-options': 'nosniff',
  'x-frame-options': 'DENY',
  'x-xss-protection': '1; mode=block',
  'referrer-policy': 'strict-origin-when-cross-origin',
  'cache-control': 'no-store, no-cache, must-revalidate',
};

function expectSecurityHeaders(response: APIResponse): void {
  const headers = response.headers();
  for (const [name, value] of Object.entries(SECURITY_HEADERS)) {
    expect(headers[name], `header ${name}`).toBe(value);
  }
}

test.describe('security headers on middleware-covered responses', () => {
  // /error/* pages intentionally answer with the matching error status
  // (see error-pages.spec.ts); the middleware still adds the headers.
  const coveredPaths: Array<[path: string, status: number]> = [
    ['/health_check', 200],
    ['/public/login', 200],
    ['/public/about', 200],
    ['/metrics', 200],
    ['/static/css/base_webapp_9.css', 200],
    ['/error/401', 401],
  ];
  for (const [path, status] of coveredPaths) {
    test(`GET ${path} carries the full security header set`, async ({ request }) => {
      const response = await request.get(path);
      expect(response.status()).toBe(status);
      expectSecurityHeaders(response);
    });
  }

  test('405 responses carry the security headers (router rejects inside the stack)', async ({
    request,
  }) => {
    // The 405 is produced by the axum router, which sits inside the
    // middleware stack, so the headers are added.
    const response = await request.post('/public/about', {
      headers: { origin: baseOrigin() },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(405);
    expectSecurityHeaders(response);
  });

  // Former documented gaps, now covered.
  test('the fallback 404 carries the full security header set', async ({ request }) => {
    // main.rs merges the 404 fallback as a sub-router with its own
    // security_headers layer (kept OUTSIDE the session layer, so a 404
    // must not create a session — see http-behavior.spec.ts).
    const response = await request.get('/definitely/not/a/real/route');
    expect(response.status()).toBe(404);
    expectSecurityHeaders(response);
  });

  test('the CSRF 403 carries the full security header set', async ({ request }) => {
    // security_headers is the outermost middleware, so a rejected
    // state-changing request still gets the headers.
    const response = await request.post('/public/login', {
      form: { username: 'x', password: 'y' },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(403);
    expectSecurityHeaders(response);
  });
});
