/**
 * Unit tests for the pure parsing helpers in utils/env.ts.
 *
 * The helpers take an explicit env-like record (defaulting to process.env)
 * precisely so they can be tested here without touching the real
 * environment.
 */
import { test, expect } from '@playwright/test';
import {
  adminCredentials,
  allowRegister,
  baseOrigin,
  ENV,
  isTruthyFlag,
  loginCredentials,
  randomPassword,
  randomUsername,
} from '../utils/env';

test.describe('baseOrigin', () => {
  test('defaults to the live deployment origin', () => {
    expect(baseOrigin({})).toBe('https://mk.mediakraken.media');
  });

  test('reduces a full URL to its origin', () => {
    expect(baseOrigin({ [ENV.baseUrl]: 'https://mk.mediakraken.media/some/path?x=1' })).toBe(
      'https://mk.mediakraken.media',
    );
  });

  test('keeps non-standard ports', () => {
    expect(baseOrigin({ [ENV.baseUrl]: 'http://localhost:8080' })).toBe('http://localhost:8080');
  });

  test('falls back to the raw value when it is not a parseable URL', () => {
    expect(baseOrigin({ [ENV.baseUrl]: 'not a url' })).toBe('not a url');
  });
});

test.describe('loginCredentials / adminCredentials', () => {
  test('return null when the username is missing', () => {
    expect(loginCredentials({ [ENV.testPassword]: 'secret-password' })).toBeNull();
  });

  test('return null when the password is missing', () => {
    expect(loginCredentials({ [ENV.testUsername]: 'someuser' })).toBeNull();
  });

  test('return the credentials when both are set', () => {
    expect(
      loginCredentials({
        [ENV.testUsername]: ' someuser ',
        [ENV.testPassword]: 'secret-password',
      }),
    ).toEqual({ username: 'someuser', password: 'secret-password' });
  });

  test('include the Authy token only when it is set', () => {
    const without = loginCredentials({
      [ENV.testUsername]: 'u',
      [ENV.testPassword]: 'p',
    });
    expect(without?.authyToken).toBeUndefined();

    const withToken = loginCredentials({
      [ENV.testUsername]: 'u',
      [ENV.testPassword]: 'p',
      [ENV.testAuthyToken]: ' 123456 ',
    });
    expect(withToken).toEqual({ username: 'u', password: 'p', authyToken: '123456' });
  });

  test('admin credentials use the ADMIN_* variables', () => {
    expect(
      adminCredentials({
        [ENV.adminUsername]: 'admin',
        [ENV.adminPassword]: 'admin-password',
        [ENV.adminAuthyToken]: '654321',
      }),
    ).toEqual({ username: 'admin', password: 'admin-password', authyToken: '654321' });
    // TEST_* variables must not leak into the admin credentials.
    expect(
      adminCredentials({
        [ENV.testUsername]: 'testuser',
        [ENV.testPassword]: 'test-password',
      }),
    ).toBeNull();
  });
});

test.describe('isTruthyFlag', () => {
  for (const value of ['1', 'true', 'yes', 'on', 'TRUE', ' On ', 'YeS']) {
    test(`accepts "${value}"`, () => {
      expect(isTruthyFlag(value)).toBe(true);
    });
  }

  for (const value of ['0', 'false', 'no', 'off', '', '  ', 'maybe', '2']) {
    test(`rejects "${value}"`, () => {
      expect(isTruthyFlag(value)).toBe(false);
    });
  }

  test('rejects undefined', () => {
    expect(isTruthyFlag(undefined)).toBe(false);
  });
});

test.describe('allowRegister', () => {
  test('defaults to off', () => {
    expect(allowRegister({})).toBe(false);
  });

  for (const value of ['1', 'true', 'yes', 'on']) {
    test(`accepts "${value}"`, () => {
      expect(allowRegister({ [ENV.allowRegister]: value })).toBe(true);
    });
  }

  test('rejects other values', () => {
    expect(allowRegister({ [ENV.allowRegister]: '0' })).toBe(false);
    expect(allowRegister({ [ENV.allowRegister]: 'please' })).toBe(false);
  });
});

test.describe('randomUsername', () => {
  test('has the mktest_ prefix and a random 6-char suffix by default', () => {
    const username = randomUsername({});
    expect(username).toMatch(/^mktest_[a-z0-9]{6}$/);
  });

  test('is deterministic when MK_TEST_RUN_ID is set', () => {
    const env = { [ENV.runId]: 'abc123' };
    expect(randomUsername(env)).toBe('mktest_abc123');
    expect(randomUsername(env)).toBe('mktest_abc123');
  });

  test('stays within the 64-char server limit for long run ids', () => {
    const env = { [ENV.runId]: 'x'.repeat(200) };
    const username = randomUsername(env);
    expect(username.length).toBe(64);
    expect(username.startsWith('mktest_')).toBe(true);
  });

  test('produces server-legal characters (3-64 chars, [A-Za-z0-9_-.@])', () => {
    for (let i = 0; i < 20; i += 1) {
      const username = randomUsername({});
      expect(username).toMatch(/^[A-Za-z0-9_.-@]{3,64}$/);
    }
  });
});

test.describe('randomPassword', () => {
  test('is deterministic for a given username', () => {
    expect(randomPassword('mktest_abc')).toBe(randomPassword('mktest_abc'));
  });

  test('satisfies the 10-128 char server limit', () => {
    const password = randomPassword('mktest_abc');
    expect(password.length).toBeGreaterThanOrEqual(10);
    expect(password.length).toBeLessThanOrEqual(128);
    expect(password.startsWith('mktest-pw-')).toBe(true);
  });

  test('stays within 128 chars for the longest possible username', () => {
    expect(randomPassword('a'.repeat(64)).length).toBeLessThanOrEqual(128);
  });
});
