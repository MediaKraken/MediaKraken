/**
 * Minimal example test / wiring smoke check.
 *
 * Hitting /health_check with the default `request` fixture proves that the
 * baseURL, the .env loading and the network path work before anything more
 * specific runs. (integration/health-check.spec.ts covers the endpoint in
 * depth.) Copy this shape when adding a new spec:
 *
 *   import { test, expect } from '../fixtures/auth';
 *
 *   test('...', async ({ page, request }) => { ... });
 */
import { test, expect } from '../fixtures/auth';

test('app is reachable and /health_check answers 200', async ({ request }) => {
  const response = await request.get('/health_check');
  expect(response.status()).toBe(200);
});
