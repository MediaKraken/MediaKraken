/**
 * Session cookie behavior, verified against the live deployment:
 *
 * - the session cookie is named "session" (the backing table is mm_session)
 * - a login (success OR failure) sets:
 *     session     HttpOnly; SameSite=Lax;   Secure; Path=/; Expires=+1h
 *     axum-flash  HttpOnly; SameSite=Strict; Secure; Path=/; Max-Age=600
 * - a third cookie "store" is always set empty with Max-Age=0 (cleared)
 * - /public/logout destroys the server-side session and clears the cookie
 *   (Max-Age=0); the old session id cannot authenticate afterwards
 *
 * IMPORTANT: /public/logout destroys the worker's shared session, so the
 * logout test performs its own dedicated login in its own per-test request
 * context and never touches the auth fixtures. The success/logout tests
 * need TEST_USERNAME / TEST_PASSWORD and skip otherwise.
 */
import type { APIResponse } from '@playwright/test';
import { test, expect } from '../fixtures/auth';
import { baseOrigin, loginCredentials } from '../utils/env';

const ORIGIN = baseOrigin();

interface SetCookie {
  name: string;
  value: string;
  attrs: Record<string, string>;
}

/** Parses every Set-Cookie header of a response into name/value/attrs. */
function setCookies(response: APIResponse): SetCookie[] {
  return response
    .headersArray()
    .filter((header) => header.name.toLowerCase() === 'set-cookie')
    .map((header) => {
      const parts = header.value.split(';').map((part) => part.trim());
      const [first = ''] = parts;
      const eq = first.indexOf('=');
      const name = eq === -1 ? first : first.slice(0, eq);
      const value = eq === -1 ? '' : first.slice(eq + 1);
      const attrs: Record<string, string> = {};
      for (const part of parts.slice(1)) {
        const i = part.indexOf('=');
        if (i === -1) {
          attrs[part.toLowerCase()] = 'true';
        } else {
          attrs[part.slice(0, i).toLowerCase()] = part.slice(i + 1);
        }
      }
      return { name, value, attrs };
    });
}

function findCookie(cookies: SetCookie[], name: string): SetCookie {
  const cookie = cookies.find((c) => c.name === name);
  expect(cookie, `Set-Cookie for "${name}"`).toBeDefined();
  return cookie as SetCookie;
}

/** Attributes every login response must carry on the session cookie. */
function expectSessionCookie(cookie: SetCookie): void {
  expect(cookie.value.length).toBeGreaterThan(0);
  expect(cookie.attrs['httponly']).toBeTruthy();
  expect(cookie.attrs['secure']).toBeTruthy();
  // The server sends "SameSite=Lax"; compare case-insensitively.
  expect(cookie.attrs['samesite']?.toLowerCase()).toBe('lax');
  expect(cookie.attrs['path']).toBe('/');
  // The 1h session lifetime is carried as Expires (no Max-Age is set).
  const expires = Date.parse(cookie.attrs['expires']);
  expect(expires, `Expires header value: ${cookie.attrs['expires']}`).not.toBeNaN();
  expect(expires).toBeGreaterThan(Date.now() + 30 * 60_000);
  expect(expires).toBeLessThan(Date.now() + 90 * 60_000);
}

/** Attributes every login response must carry on the flash cookie. */
function expectFlashCookie(cookie: SetCookie): void {
  expect(cookie.attrs['httponly']).toBeTruthy();
  expect(cookie.attrs['secure']).toBeTruthy();
  // The server sends "SameSite=Strict"; compare case-insensitively.
  expect(cookie.attrs['samesite']?.toLowerCase()).toBe('strict');
  expect(cookie.attrs['path']).toBe('/');
  expect(cookie.attrs['max-age']).toBe('600');
}

test.describe('login responses set secure cookies (always runs)', () => {
  test('a failed login sets the session and flash cookies with secure attributes', async ({
    request,
  }) => {
    const response = await request.post('/public/login', {
      form: { username: 'no_such_user_autotest', password: 'definitely-wrong-pw' },
      headers: { origin: ORIGIN },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(303);
    expect(response.headers()['location']).toBe('/public/login');

    const cookies = setCookies(response);
    expectSessionCookie(findCookie(cookies, 'session'));
    expectFlashCookie(findCookie(cookies, 'axum-flash'));
    // The flash error message is stored IN the signed flash cookie.
    expect(findCookie(cookies, 'axum-flash').value).toContain(
      'Unknown%20user%20or%20password%20incorrect',
    );
  });
});

test.describe('successful login cookies (requires TEST_USERNAME / TEST_PASSWORD)', () => {
  test('a successful login sets the same secure cookie set', async ({ request }) => {
    const credentials = loginCredentials();
    test.skip(!credentials, 'TEST_USERNAME / TEST_PASSWORD not set');
    if (!credentials) {
      return;
    }

    const response = await request.post('/public/login', {
      form: {
        username: credentials.username,
        password: credentials.password,
        ...(credentials.authyToken ? { authy_token: credentials.authyToken } : {}),
      },
      headers: { origin: ORIGIN },
      maxRedirects: 0,
    });
    expect(response.status()).toBe(303);
    expect(response.headers()['location']).toBe('/user/home');

    const cookies = setCookies(response);
    expectSessionCookie(findCookie(cookies, 'session'));
    expectFlashCookie(findCookie(cookies, 'axum-flash'));
  });
});

test.describe('logout (requires TEST_USERNAME / TEST_PASSWORD)', () => {
  test('logout destroys the server-side session', async ({ request }) => {
    const credentials = loginCredentials();
    test.skip(!credentials, 'TEST_USERNAME / TEST_PASSWORD not set');
    if (!credentials) {
      return;
    }

    // 1) Dedicated login in this test's own request context.
    const login = await request.post('/public/login', {
      form: {
        username: credentials.username,
        password: credentials.password,
        ...(credentials.authyToken ? { authy_token: credentials.authyToken } : {}),
      },
      headers: { origin: ORIGIN },
      maxRedirects: 0,
    });
    expect(login.status()).toBe(303);
    expect(login.headers()['location']).toBe('/user/home');
    expect(setCookies(login).some((c) => c.name === 'session' && c.value.length > 0));

    // 2) The session authenticates (same contract as the authState fixture,
    //    which requires /user/home -> 200 for the configured account).
    const before = await request.get('/user/home');
    expect(before.status()).toBe(200);

    // 3) Logout: 303 to the login page and the session cookie is cleared.
    const logout = await request.get('/public/logout', { maxRedirects: 0 });
    expect(logout.status()).toBe(303);
    expect(logout.headers()['location']).toBe('/public/login');
    const cleared = findCookie(setCookies(logout), 'session');
    expect(cleared.value).toBe('');
    expect(cleared.attrs['max-age']).toBe('0');

    // 4) The session cannot be reused: with the cookie gone (and the
    //    server-side row destroyed) the dashboard no longer authenticates.
    const after = await request.get('/user/home');
    expect(after.status()).toBe(401);
  });

  test('GET /public/logout without a session still redirects to login', async ({
    request,
  }) => {
    const response = await request.get('/public/logout', { maxRedirects: 0 });
    expect(response.status()).toBe(303);
    expect(response.headers()['location']).toBe('/public/login');
  });
});
