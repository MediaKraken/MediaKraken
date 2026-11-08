/**
 * Environment access for the MediaKraken autotest suite.
 *
 * All parsing logic lives in pure functions that take an env-like record so
 * they can be unit-tested without touching process.env (see unit/utils.spec.ts).
 */

export interface EnvLike {
  [key: string]: string | undefined;
}

/** Environment variable names used by the suite (single source of truth). */
export const ENV = {
  baseUrl: 'BASE_URL',
  testUsername: 'TEST_USERNAME',
  testPassword: 'TEST_PASSWORD',
  testAuthyToken: 'TEST_AUTHY_TOKEN',
  adminUsername: 'ADMIN_USERNAME',
  adminPassword: 'ADMIN_PASSWORD',
  adminAuthyToken: 'ADMIN_AUTHY_TOKEN',
  allowRegister: 'MK_TEST_ALLOW_REGISTER',
  runId: 'MK_TEST_RUN_ID',
} as const;

export const DEFAULT_BASE_URL = 'https://mk.mediakraken.media';

/**
 * Returns the `scheme://host[:port]` origin of BASE_URL.
 * Falls back to the raw value when it is not a parseable URL.
 */
export function baseOrigin(env: EnvLike = process.env): string {
  const raw = env[ENV.baseUrl] ?? DEFAULT_BASE_URL;
  try {
    return new URL(raw).origin;
  } catch {
    return raw;
  }
}

export interface Credentials {
  username: string;
  password: string;
  /** Present only when a 2FA (Authy) token was configured. */
  authyToken?: string;
}

function credentialsFrom(
  env: EnvLike,
  userKey: string,
  passKey: string,
  tokenKey?: string,
): Credentials | null {
  const username = env[userKey]?.trim();
  const password = env[passKey];
  if (!username || password === undefined) {
    return null;
  }
  const credentials: Credentials = { username, password };
  const token = tokenKey ? env[tokenKey]?.trim() : undefined;
  if (token) {
    credentials.authyToken = token;
  }
  return credentials;
}

/** Credentials for the regular (non-admin) user, or null when not configured. */
export function loginCredentials(env: EnvLike = process.env): Credentials | null {
  return credentialsFrom(env, ENV.testUsername, ENV.testPassword, ENV.testAuthyToken);
}

/** Credentials for the admin user, or null when not configured. */
export function adminCredentials(env: EnvLike = process.env): Credentials | null {
  return credentialsFrom(env, ENV.adminUsername, ENV.adminPassword, ENV.adminAuthyToken);
}

/** Truthy parsing for opt-in flags: "1", "true", "yes", "on" (case/space insensitive). */
export function isTruthyFlag(value: string | undefined): boolean {
  if (value === undefined) {
    return false;
  }
  return ['1', 'true', 'yes', 'on'].includes(value.trim().toLowerCase());
}

/**
 * Successful registration actually inserts a user row (and the first real
 * user is promoted to admin server-wide), so it only runs when explicitly enabled.
 */
export function allowRegister(env: EnvLike = process.env): boolean {
  return isTruthyFlag(env[ENV.allowRegister]);
}

/**
 * Username for registration tests. Must satisfy the server rules:
 * 3-64 chars, only [A-Za-z0-9_-.@]. A random suffix keeps runs unique.
 */
export function randomUsername(env: EnvLike = process.env): string {
  const suffix = env[ENV.runId] ?? Math.random().toString(36).slice(2, 8);
  return `mktest_${suffix}`.slice(0, 64);
}

/** Password for registration tests. Must be 10-128 chars. */
export function randomPassword(username: string): string {
  return `mktest-pw-${username}`.slice(0, 128);
}
