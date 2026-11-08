# MediaKraken Autotest

Playwright-based tests for the MediaKraken web app (`docker/core/mkwebaxum`).

The suite targets the **live deployment** (`https://mk.mediakraken.media`) by
default and is designed to be **green out of the box**: tests that need real
credentials or that would mutate production data skip themselves when the
relevant environment variables are not set.

## Project structure

```
autotest/
├── e2e/             # Full user flows (login, register, authed pages, ...)
├── integration/     # HTTP behavior: health, metrics, static, API, errors
├── unit/            # Unit tests for the utils/ helpers
├── tests/           # Minimal example spec (wiring smoke check)
├── fixtures/        # Auth fixtures (worker-scoped login state, ...)
├── utils/           # env.ts (env parsing) + auth.ts (login helpers)
├── playwright.config.ts
└── .env.example     # copy to .env
```

All specs import `{ test, expect }` from `../fixtures/auth` — the custom
`test` object adds the auth fixtures and re-exports Playwright's `expect`.

## Prerequisites

- Node.js ≥ 18 (LTS recommended)
- npm ≥ 9
- Reachable MediaKraken backend (default: the live deployment)

## Installation

```bash
cd autotest
npm install
npx playwright install          # browsers (chromium, firefox, webkit)
# optional: npx playwright install-deps   # system libraries for browsers
```

## Running tests

```bash
npx playwright test                      # all projects (4 browsers)
npx playwright test --project=chromium   # one browser
npx playwright test e2e/                 # one directory
npx playwright test unit/                # offline unit tests only
npx playwright test --headed             # visible browser
npx playwright test --debug              # Playwright Inspector
npx playwright show-report               # HTML report after a run
```

- Locally: 4 parallel workers, no retries, HTML reporter.
- On CI (`CI=1`): 1 worker, 2 retries, `forbidOnly` enforced.
- Global timeout: 60s per test (live network round trips).

## Configuration (`.env`)

Copy `.env.example` to `.env`. Everything is optional:

| Variable | Used for | When unset |
| --- | --- | --- |
| `BASE_URL` | target origin (baseURL + CSRF Origin/Referer) | defaults to `https://mk.mediakraken.media` |
| `TEST_USERNAME` / `TEST_PASSWORD` | regular user for authed tests | all `authPage`/`authRequest` tests **skip** |
| `TEST_AUTHY_TOKEN` | 2FA token for the test user | omitted from the login form |
| `ADMIN_USERNAME` / `ADMIN_PASSWORD` | admin user for admin tests | all `adminPage`/`adminRequest` tests **skip** |
| `ADMIN_AUTHY_TOKEN` | 2FA token for the admin | omitted from the login form |
| `MK_TEST_ALLOW_REGISTER` | `1` enables registration *success* tests | success tests **skip** (validation tests still run) |
| `MK_TEST_RUN_ID` | deterministic suffix for generated usernames | random 6-char suffix |

Notes:

- The test user must actually have the `User::View` permission — the fixture
  verifies `GET /user/home → 200` and fails with a clear message otherwise.
  (Freshly registered accounts start with **no** permissions and login 303s
  fine but every `/user/*` page then 401s.)
- The admin user must have `Admin::View`; the fixture verifies
  `GET /admin/user/1 → 200` (not `/admin/home`, see below).

## Running against a local instance

Point `BASE_URL` at the local app. Two server-side constraints matter:

- The session cookies are `Secure`. Browsers treat `http://localhost` as a
  secure context, so local runs work on `http://localhost:8080` (use a real
  HTTPS origin for anything else).
- The CSRF middleware requires the origin you submit from to be in the
  server's `MKWEBAPP_ALLOWED_ORIGINS`. Make sure the local origin is included
  in that server environment variable, or every form POST 403s.

## Documented bugs the suite codifies

These tests assert **current, known-wrong** behavior on purpose so the suite
goes red (and needs updating) if the bug is ever fixed:

| Test file | Documented issue |
| --- | --- |
| `integration/api-titlesearch.spec.ts` | `/api/titlesearch/*` 500s for everyone — route registered after `AuthSessionLayer`, handler still extracts `AuthSession` |
| `e2e/error-pages.spec.ts`, `e2e/admin-access.spec.ts` | `/admin/*` rejections render the 403 template but return a **401 status** |
| `e2e/security-headers.spec.ts`, `integration/http-behavior.spec.ts` | the fallback 404 (and the CSRF 403) bypass the security-headers middleware |
| `e2e/auth-rate-limit.spec.ts` | the auth rate limiter is **inert**: `axum::serve` without `into_make_service_with_connect_info` never sets `ConnectInfo`, so `rate_limit_auth` always passes through |
| `e2e/user-search.spec.ts` | `/user/search` is wired to the public stub handler (always empty results); the permission-gated handler is not registered |
| `fixtures/auth.ts` (comment) | `/admin/home` additionally calls myexternalip.com with `.unwrap()` — the fixture verifies `/admin/user/1` instead |

Also noted in `e2e/session-cookies.spec.ts`: the session cookie is named
`session`, carries its 1h lifetime as `Expires` (no `Max-Age`), and the
`axum-flash` cookie is `SameSite=Strict` with `Max-Age=600`.

## CI

```bash
npm ci
npx playwright install --with-deps chromium
CI=1 npx playwright test --project=chromium --reporter=github
```

Keep `MK_TEST_ALLOW_REGISTER` unset in CI against production: the success
tests insert real user rows.
