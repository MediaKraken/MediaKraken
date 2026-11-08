/**
 * CSRF protection (require_same_origin middleware in main.rs).
 *
 * State-changing methods (POST/PUT/PATCH/DELETE) must carry an Origin header
 * (or a Referer fallback) that matches one of the entries in
 * MKWEBAPP_ALLOWED_ORIGINS (comma-separated `scheme://host[:port]` values).
 * The matcher accepts an exact origin or the origin followed by a path
 * (rest starting with `/`), which is what keeps suffix look-alikes
 * (`https://mk.mediakraken.media.evil.com`) rejected.
 *
 * All tests here are safe to run against the live deployment: the rejected
 * requests never reach a handler, and the requests that do pass the gate
 * submit invalid credentials or invalid registration data (no state change).
 */
import { test, expect } from '../fixtures/auth';
import { baseOrigin } from '../utils/env';

const REJECTION_BODY = 'cross-origin request rejected';
const ORIGIN = baseOrigin();

test.describe('rejected state-changing requests', () => {
  test('POST without Origin or Referer is rejected with 403', async ({ request }) => {
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(403);
    expect(response.headers()['content-type']).toContain('text/plain');
    expect((await response.text()).trim()).toBe(REJECTION_BODY);
  });

  test('POST with a foreign Origin is rejected with 403', async ({ request }) => {
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      headers: { origin: 'https://evil.example.com' },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(403);
    expect((await response.text()).trim()).toBe(REJECTION_BODY);
  });

  test('POST with a suffix look-alike origin is rejected with 403', async ({ request }) => {
    // The origin shares the allowed origin as a PREFIX but the remainder
    // does not start with "/", so the matcher must not accept it.
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      headers: { origin: `${ORIGIN}.evil.example.com` },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(403);
    expect((await response.text()).trim()).toBe(REJECTION_BODY);
  });

  test('DELETE without Origin is rejected with 403', async ({ request }) => {
    // DELETE is a state-changing method and is gated like POST/PUT/PATCH.
    const response = await request.delete('/public/about', { maxRedirects: 0 });
    expect(response.status()).toBe(403);
    expect((await response.text()).trim()).toBe(REJECTION_BODY);
  });
});

test.describe('accepted requests', () => {
  test('GET without Origin passes the gate', async ({ request }) => {
    // GET is not state-changing; the middleware passes it through.
    const response = await request.get('/public/about');
    expect(response.status()).toBe(200);
  });

  test('POST with the app origin reaches the handler', async ({ request }) => {
    // Bad credentials -> 303 back to /public/login, which proves the CSRF
    // gate let the request through to the login handler.
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      headers: { origin: ORIGIN },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(303);
    expect(response.headers()['location']).toBe('/public/login');
  });

  test('POST with only a matching Referer (no Origin) reaches the handler', async ({
    request,
  }) => {
    // The middleware falls back to Referer when Origin is absent.
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      headers: { referer: `${ORIGIN}/public/login` },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(303);
    expect(response.headers()['location']).toBe('/public/login');
  });
});
