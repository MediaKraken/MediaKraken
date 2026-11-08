/**
 * Auth rate limiting — documents the CURRENT behavior.
 *
 * main.rs configures per-IP limiters for POST /public/login (10 requests /
 * 60s) and POST /public/register (5 / 60s), but the limiter is INERT on the
 * current deployment: the app is served with a plain `axum::serve(listener,
 * app)` (no `into_make_service_with_connect_info`), so the
 * `ConnectInfo<SocketAddr>` extension is never set and `rate_limit_auth`
 * takes its `None => return next.run(req)` branch for every request.
 *
 * These tests assert that rapid auth POSTs are therefore NOT rejected with
 * 429. The counts are deliberately kept under the configured limits so the
 * suite would still pass if the limiter were ever activated.
 *
 * If the limiter is fixed, flip the expectations: the first N requests
 * should succeed and the (N+1)th should return 429
 * ("too many requests, please try again later").
 */
import { test, expect } from '../fixtures/auth';
import { baseOrigin } from '../utils/env';

const ORIGIN = baseOrigin();

test.describe('auth rate limiting (limiter currently inert)', () => {
  // Keep the two bursts from interleaving with each other within this file.
  test.describe.configure({ mode: 'serial' });

  test('rapid login POSTs are not rejected with 429', async ({ request }) => {
    const statuses: number[] = [];
    for (let i = 0; i < 5; i += 1) {
      const response = await request.post('/public/login', {
        form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
        headers: { origin: ORIGIN },
        maxRedirects: 0,
      });
      statuses.push(response.status());
    }
    // Invalid credentials -> 303 back to /public/login; a 429 would mean the
    // limiter became active.
    expect(statuses).toEqual([303, 303, 303, 303, 303]);
  });

  test('rapid register POSTs are not rejected with 429', async ({ request }) => {
    const statuses: number[] = [];
    for (let i = 0; i < 4; i += 1) {
      const response = await request.post('/public/register', {
        // Validation failure -> 303 back to /public/register, no user row.
        form: { username: 'ab', password: 'short' },
        headers: { origin: ORIGIN },
        maxRedirects: 0,
      });
      statuses.push(response.status());
    }
    expect(statuses).toEqual([303, 303, 303, 303]);
  });
});
