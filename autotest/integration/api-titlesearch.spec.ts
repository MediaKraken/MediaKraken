/**
 * /api/titlesearch/{title}
 *
 * The route is registered at the TOP of the route chain in main.rs, in
 * front of the AuthSession/Session layers, so the handler's `AuthSession`
 * extractor works: anonymous visitors get the 401 error page, and any
 * authenticated user with User::View gets the search results page
 * (bss_api/bss_api_title_search.html — one node per media type).
 */
import { test, expect } from '../fixtures/auth';

test.describe('/api/titlesearch', () => {
  test('anonymous request gets the 401 error page', async ({ request }) => {
    const response = await request.get('/api/titlesearch/test');
    expect(response.status()).toBe(401);
    expect(response.headers()['content-type']).toContain('text/html');
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken – 401</title>');
    expect(body).toContain('401 – Must be a verified user');
  });

  test('authenticated request renders the title search results page', async ({
    authRequest,
  }) => {
    // A made-up title matches no metadata rows, so all three result nodes
    // render empty — the point is that the handler runs end to end.
    const response = await authRequest.get('/api/titlesearch/autotest-no-such-title-xyz');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/html');
    const body = await response.text();
    expect(body).toContain('<title>MediaKraken API</title>');
    expect(body).toContain('id="the-movie-node"');
    expect(body).toContain('id="the-tv-node"');
    expect(body).toContain('id="the-music-node"');
  });

  test('trailing slash redirects to the canonical route (401 for anonymous)', async ({
    request,
  }) => {
    // route_with_tsr registers the trailing-slash variant as a 308
    // redirect to the canonical path, which then 401s for anonymous.
    const redirected = await request.get('/api/titlesearch/test/', { maxRedirects: 0 });
    expect(redirected.status()).toBe(308);
    expect(redirected.headers()['location']).toBe('/api/titlesearch/test');

    const canonical = await request.get('/api/titlesearch/test', { maxRedirects: 0 });
    expect(canonical.status()).toBe(401);
  });
});
