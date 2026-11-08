/**
 * Security headers (security_headers middleware in main.rs).
 *
 * The middleware inserts a fixed header set into every response that passes
 * through it and strips the app's Server header. Two responses bypass it
 * entirely and are asserted as documented gaps:
 *
 * - the app-level fallback 404 (attached via app.fallback() AFTER the
 *   middleware stack), and
 * - the CSRF 403 (require_same_origin is the OUTERMOST middleware, so it
 *   rejects before security_headers runs).
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

function expectNoSecurityHeaders(response: APIResponse): void {
  const headers = response.headers();
  for (const name of Object.keys(SECURITY_HEADERS)) {
    expect(headers[name], `header ${name} should be absent`).toBeUndefined();
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
});

test.describe('responses that bypass the middleware (documented gaps)', () => {
  test('the fallback 404 carries NO security headers', async ({ request }) => {
    // main.rs attaches app.fallback() after .layer(...), so fallback
    // responses never reach security_headers.
    const response = await request.get('/definitely/not/a/real/route');
    expect(response.status()).toBe(404);
    expectNoSecurityHeaders(response);
  });

  test('the CSRF 403 carries NO security headers', async ({ request }) => {
    // require_same_origin is the outermost middleware: a rejected
    // state-changing request never reaches security_headers.
    const response = await request.post('/public/login', {
      form: { username: 'x', password: 'y' },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(403);
    expectNoSecurityHeaders(response);
  });
});
