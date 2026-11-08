/**
 * Auth rate limiting (active limiter).
 *
 * main.rs rate-limits the auth POST endpoints per client IP in a sliding
 * 60s window:
 *
 *   POST /public/login + /public/forgot_password  ->  10 requests
 *   POST /public/register                          ->   5 requests
 *
 * The client IP is the first X-Forwarded-For hop (the app is served
 * behind the envoy ingress, whose ConnectInfo peer is shared by all
 * clients), falling back to the TCP peer. The limiter is in-memory PER
 * REPLICA and the deployment runs 3 replicas behind the load balancer, so
 * a burst must exceed replicas x limit to be rejected even under a
 * perfectly even spread. The bursts below send up to that worst-case
 * count and assert that:
 *
 * - every response BEFORE the rejection is a normal 303 (the handler ran),
 * - the LAST response is 429 "too many requests, please try again later",
 * - the 429 carries the full security header set.
 *
 * Concurrent consumers from the same client IP can only make the 429
 * arrive earlier, never later.
 *
 * IMPORTANT: this spec exhausts the shared-IP login/register budget for
 * the next 60s, so it runs in its own Playwright project (`rate-limit`)
 * and must NOT run in parallel with the browser projects, which POST to
 * the same endpoints and expect 303s (see playwright.config.ts and the
 * README CI section).
 */
import type { APIRequestContext, APIResponse } from '@playwright/test';
import { test, expect } from '../fixtures/auth';
import { baseOrigin } from '../utils/env';

const ORIGIN = baseOrigin();
const RATE_LIMIT_BODY = 'too many requests, please try again later';

// 3 replicas x 10 login requests (worst-case even spread).
const LOGIN_BURST_MAX = 40;
// 3 replicas x 5 register requests (worst-case even spread).
const REGISTER_BURST_MAX = 20;

const SECURITY_HEADERS: Record<string, string> = {
  'x-content-type-options': 'nosniff',
  'x-frame-options': 'DENY',
  'x-xss-protection': '1; mode=block',
  'referrer-policy': 'strict-origin-when-cross-origin',
  'cache-control': 'no-store, no-cache, must-revalidate',
};

async function expectRateLimitResponse(response: APIResponse): Promise<void> {
  expect(response.status()).toBe(429);
  for (const [name, value] of Object.entries(SECURITY_HEADERS)) {
    expect(response.headers()[name], `header ${name}`).toBe(value);
  }
  expect((await response.text()).trim()).toBe(RATE_LIMIT_BODY);
}

/**
 * POSTs `form` to `path` up to `maxAttempts` times and returns the first
 * 429. Every earlier response must be a 303 (bad data -> handler answers
 * with a redirect, no state change on the server).
 */
async function burstUntilRateLimited(
  request: APIRequestContext,
  path: string,
  form: Record<string, string>,
  maxAttempts: number,
): Promise<{ passthrough: number; limited: APIResponse }> {
  for (let i = 0; i < maxAttempts; i += 1) {
    const response = await request.post(path, {
      form,
      headers: { origin: ORIGIN },
      maxRedirects: 0,
    });
    if (response.status() === 429) {
      return { passthrough: i, limited: response };
    }
    const snippet = (await response.text().catch(() => '')).slice(0, 120);
    expect(
      response.status(),
      `attempt ${i + 1} of ${path}: expected 303 before the 429 (body: ${snippet || 'empty'})`,
    ).toBe(303);
  }
  throw new Error(
    `${path} was not rate-limited after ${maxAttempts} attempts; the limiter ` +
      'may be inactive, the replica count may have changed, or X-Forwarded-For ' +
      'keying may not be in effect on this deployment',
  );
}

// The two bursts target separate limiters (login vs register) and run in
// order: the `rate-limit` project uses workers: 1 and fullyParallel: false,
// so tests in this file never interleave. They are deliberately NOT put in
// a serial group, so a failure in one still lets the other report.
test.describe('auth rate limiting (active limiter)', () => {
  test('a login burst is rejected with a 429 carrying the security headers', async ({
    request,
  }) => {
    const { limited } = await burstUntilRateLimited(
      request,
      '/public/login',
      // Unknown user -> 303 back to /public/login; no account is touched.
      { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      LOGIN_BURST_MAX,
    );
    await expectRateLimitResponse(limited);
  });

  test('a register burst is rejected with a 429 carrying the security headers', async ({
    request,
  }) => {
    const { limited } = await burstUntilRateLimited(
      request,
      '/public/register',
      // Validation failure (short username) -> 303 back to /public/register;
      // no user row is created.
      { username: 'ab', password: 'short' },
      REGISTER_BURST_MAX,
    );
    await expectRateLimitResponse(limited);
  });
});
