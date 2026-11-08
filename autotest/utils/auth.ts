/**
 * Login helpers for the MediaKraken web app.
 *
 * The server rate-limits POST /public/login to 10 attempts per IP per 60s
 * (and POST /public/register to 5), and state-changing requests are only
 * accepted with a matching Origin/Referer (CSRF middleware). The helpers
 * here cope with both: they send the app origin as Origin header and back
 * off when they hit a 429.
 */
import type { APIRequestContext, Page, APIResponse } from '@playwright/test';
import { baseOrigin } from './env';

export const RATE_LIMIT_WINDOW_MS = 61_000; // server window is 60s, add 1s slack
const MAX_RATE_LIMIT_WAITS = 2;

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

export interface LoginResult {
  /** True when the server answered with a redirect to /user/home. */
  ok: boolean;
  status: number;
  location: string | null;
  error: string | null;
}

function endsWithHome(location: string | null): boolean {
  if (!location) {
    return false;
  }
  return location.replace(/\/+$/, '').endsWith('/user/home');
}

async function describeFailure(response: APIResponse): Promise<string> {
  const body = (await response.text().catch(() => '')).slice(0, 200);
  return `status ${response.status} (body: ${body || 'empty'})`;
}

/**
 * POSTs the login form directly (no full page load) without following the
 * redirect, so the caller can inspect the 303/304 location:
 *
 * - success         -> 303, location /user/home
 * - bad credentials -> 303, location /public/login (flash error cookie set)
 * - CSRF rejection  -> 403 "cross-origin request rejected"
 * - rate limited    -> 429 (waited out and retried up to MAX_RATE_LIMIT_WAITS times)
 *
 * Cookies from the response (session + flash) are stored on `request`'s
 * context, which is exactly what we want for the storage-state fixture.
 */
export async function loginViaRequest(
  request: APIRequestContext,
  username: string,
  password: string,
  authyToken?: string,
): Promise<LoginResult> {
  const form: Record<string, string> = { username, password };
  if (authyToken) {
    form.authy_token = authyToken;
  }

  for (let waits = 0; ; waits += 1) {
    const response = await request.post('/public/login', {
      form,
      headers: { origin: baseOrigin() },
      maxRedirects: 0,
    });

    if (response.status() !== 429) {
      const location = response.headers()['location'] ?? null;
      const ok =
        (response.status() === 303 || response.status() === 302) && endsWithHome(location);
      return {
        ok,
        status: response.status(),
        location,
        error: ok ? null : await describeFailure(response),
      };
    }

    if (waits >= MAX_RATE_LIMIT_WAITS) {
      return {
        ok: false,
        status: 429,
        location: null,
        error: 'still rate limited (429) after waiting out the 60s window twice',
      };
    }
    await sleep(RATE_LIMIT_WINDOW_MS);
  }
}

/**
 * Fills in and submits the actual login form through the UI.
 * Waits for the redirect to /user/home, which only happens on success.
 */
export async function loginViaPage(
  page: Page,
  username: string,
  password: string,
  authyToken?: string,
): Promise<void> {
  await page.goto('/public/login');
  await page.getByPlaceholder('Username').fill(username);
  await page.getByPlaceholder('Password').fill(password);
  if (authyToken) {
    await page.getByPlaceholder('2FA Code (if enabled)').fill(authyToken);
  }
  await page.getByRole('button', { name: 'Login' }).click();
  await page.waitForURL(/\/user\/home/);
}

/**
 * Fills in and submits the registration form through the UI.
 * The server always answers with a redirect to /public/register (errors) or
 * /public/login (success), so the caller decides which URL to wait for.
 */
export async function submitRegisterForm(
  page: Page,
  username: string,
  password: string,
): Promise<void> {
  await page.goto('/public/register');
  await page.getByPlaceholder('Username').fill(username);
  await page.getByPlaceholder('Password').fill(password);
  await page.getByRole('button', { name: 'Register Account' }).click();
}

/** Reads the red flash-error banner rendered on login/register pages. */
export async function readFlashError(page: Page): Promise<string> {
  const alert = page.locator('[role="alert"]').first();
  await alert.waitFor({ state: 'visible' });
  return alert.innerText();
}
