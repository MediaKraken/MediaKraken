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
npx playwright test                      # the 4 browser projects
npx playwright test --project=chromium   # one browser
npx playwright test --project=rate-limit # the auth rate-limit spec (separately!)
npx playwright test e2e/                 # one directory
npx playwright test unit/                # offline unit tests only
npx playwright test --headed             # visible browser
npx playwright test --debug              # Playwright Inspector
npx playwright show-report               # HTML report after a run
```

- Locally: 4 parallel workers, no retries, HTML reporter.
- On CI (`CI=1`): 1 worker, 2 retries, `forbidOnly` enforced.
- Global timeout: 60s per test (live network round trips).
- The `rate-limit` project runs `e2e/auth-rate-limit.spec.ts` on its own
  (1 worker). It deliberately exhausts the shared client-IP login/register
  budget for 60s, so it must **not** run in parallel with the browser
  projects — Playwright runs projects concurrently, so invoke it as a
  separate command, AFTER the browser projects (their login tests have no
  429 back-off and would see the depleted budget).

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

## Previously documented bugs (all fixed — specs now assert the fixed behavior)

This suite originally asserted six known-wrong behaviors on purpose. All
six are now fixed in `docker/core/mkwebaxum`, and the specs assert the
**fixed** behavior:

| Test file | Fixed behavior asserted |
| --- | --- |
| `integration/api-titlesearch.spec.ts` | the route is registered in front of the auth/session layers: anonymous → 401 error page, authenticated → 200 results page (no more 500 "Can't extract AuthSession") |
| `e2e/error-pages.spec.ts`, `e2e/admin-access.spec.ts` | `/admin/*` rejections return a **403 status** matching the 403 template |
| `e2e/security-headers.spec.ts`, `integration/http-behavior.spec.ts` | the fallback 404 and the CSRF 403 both carry the full security header set (`security_headers` is the outermost middleware; the 404 fallback is merged with its own copy of it) |
| `e2e/auth-rate-limit.spec.ts` | the limiter is **active** (`into_make_service_with_connect_info` + X-Forwarded-For keying): a burst past the per-IP budget ends in a 429 "too many requests, please try again later" carrying the security headers |
| `e2e/user-search.spec.ts` | `/user/search` is wired to the permission-gated handler: anonymous → 401 error page, `User::View` → 200 search form (query params are accepted but ignored — no Query extractor) |
| `fixtures/auth.ts` (comment) | `/admin/home` no longer `.unwrap()`s the myexternalip.com fetch (falls back to "unavailable"), so the suite no longer needs to work around it |

Until the fixed build is deployed, the specs in the table above are
**expected to be red** against the live deployment; everything else is
unchanged and must stay green.

Notes:

- The rate limits are 10 login-equivalent POSTs (login + forgot_password)
  and 5 register POSTs per 60s per client IP, in-memory **per replica**
  (the deployment runs 3 replicas, so the effective site-wide limit is
  looser). The rate-limit spec's bursts are sized to exceed the worst
  case, and concurrent consumers from the same IP can only make the 429
  arrive earlier.
- Also noted in `e2e/session-cookies.spec.ts`: the session cookie is named
`session`, carries its 1h lifetime as `Expires` (no `Max-Age`), and the
`axum-flash` cookie is `SameSite=Strict` with `Max-Age=600`.

## CI

Run the browser projects first, then the rate-limit project — the rate
limit spec exhausts the shared client-IP login/register budget for 60s,
and Playwright runs projects concurrently:

```bash
npm ci
npx playwright install --with-deps chromium
CI=1 npx playwright test --project=chromium --reporter=github
CI=1 npx playwright test --project=rate-limit --reporter=github
```

Keep `MK_TEST_ALLOW_REGISTER` unset in CI against production: the success
tests insert real user rows.
