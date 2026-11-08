/**
 * /api/titlesearch/{title}
 *
 * DOCUMENTED BUG (asserting current behavior on purpose):
 * the route is registered AFTER AuthSessionLayer in main.rs
 * (`.layer(AuthSessionLayer::...())` is applied to routes registered before
 * it; this one is added afterwards "so anyone can access" it). The handler
 * still extracts `AuthSession<...>`, which fails because the layer never ran
 * for this route, so every request — anonymous or authenticated — gets a
 * plain-text 500 with the extractor's panic message.
 *
 * If/when the route is moved in front of the layer (or the extraction
 * removed), these tests should be updated to assert the working API.
 */
import { test, expect } from '../fixtures/auth';

const EXPECTED_ERROR_BODY = "Can't extract AuthSession. Is `AuthSessionLayer` enabled?";

test.describe('/api/titlesearch', () => {
  test('anonymous request returns 500 with the extractor error', async ({ request }) => {
    const response = await request.get('/api/titlesearch/test');
    expect(response.status()).toBe(500);
    expect(response.headers()['content-type']).toContain('text/plain');
    expect((await response.text()).trim()).toBe(EXPECTED_ERROR_BODY);
  });

  test('authenticated request returns the same 500 (bug affects everyone)', async ({
    authRequest,
  }) => {
    const response = await authRequest.get('/api/titlesearch/test');
    expect(response.status()).toBe(500);
    expect((await response.text()).trim()).toBe(EXPECTED_ERROR_BODY);
  });

  test('trailing slash redirects to the canonical (broken) route', async ({ request }) => {
    // /api/titlesearch/{title}/ is not a separately registered route; Axum
    // 308-redirects it to the canonical path, which then 500s as well.
    const redirected = await request.get('/api/titlesearch/test/', { maxRedirects: 0 });
    expect(redirected.status()).toBe(308);
    expect(redirected.headers()['location']).toBe('/api/titlesearch/test');

    const canonical = await request.get('/api/titlesearch/test', { maxRedirects: 0 });
    expect(canonical.status()).toBe(500);
  });
});
