/**
 * Authenticated page smoke test: every route here requires the User::View
 * permission and must answer 200 with an HTML body for a signed-in user.
 *
 * Deliberately excluded (they perform external network calls, which would
 * make the suite depend on third-party availability):
 * - /user/internet/{flickr,twitchtv,vimeo,youtube} detail routes
 * - /user/metadata/openlibrary/* (OpenLibrary API)
 * - /admin/home (myexternalip.com lookup; see admin-access.spec.ts)
 *
 * The routes were verified against the live deployment; if a new route is
 * added to the list, make sure it is read-only with respect to external
 * services first.
 */
import { test, expect } from '../fixtures/auth';

const USER_ROUTES = [
  '/user/home',
  '/user/profile',
  '/user/queue',
  '/user/sync/1',
  '/user/hardware',
  '/user/internet',
  '/user/media/cctv',
  '/user/media/genre',
  '/user/media/image',
  '/user/media/book/1',
  '/user/media/game/1',
  '/user/media/iradio/1',
  '/user/media/movie/1',
  '/user/media/music/1',
  '/user/media/sports/1',
  '/user/media/tv/1',
  '/user/metadata/book/1',
  '/user/metadata/collection/1',
  '/user/metadata/game/1',
  '/user/metadata/movie/1',
  '/user/metadata/music/1',
  '/user/metadata/person/1',
  '/user/metadata/sports/1',
  '/user/metadata/tv/1',
  '/user/playback/audio',
  '/user/playback/comic',
  '/user/playback/video',
];

test.describe('authenticated user pages (require User::View)', () => {
  for (const path of USER_ROUTES) {
    test(`GET ${path} renders for a signed-in user`, async ({ authRequest }) => {
      const response = await authRequest.get(path);
      expect(response.status(), `${path} should be 200`).toBe(200);
      expect(response.headers()['content-type']).toContain('text/html');
      const body = await response.text();
      expect(body.length, `${path} should have a non-empty body`).toBeGreaterThan(0);
      expect(body).toContain('<html');
    });
  }

  test('anonymous access to a user page is a 401', async ({ request }) => {
    // Spot-check that the permission gate actually applies.
    const response = await request.get('/user/home');
    expect(response.status()).toBe(401);
  });
});

test.describe('authenticated page content', () => {
  test('profile page title and heading', async ({ authPage: page }) => {
    await page.goto('/user/profile');
    await expect(page).toHaveTitle('MediaKraken User Profile');
    await expect(page.getByRole('heading', { name: 'Profile', level: 1 })).toBeVisible();
  });

  test('queue page title and heading', async ({ authPage: page }) => {
    await page.goto('/user/queue');
    await expect(page).toHaveTitle('MediaKraken Media Queue');
    await expect(page.getByRole('heading', { name: 'Queue', level: 1 })).toBeVisible();
  });

  test('sync page title', async ({ authPage: page }) => {
    await page.goto('/user/sync/1');
    await expect(page).toHaveTitle('MediaKraken Sync');
  });

  test('hardware page title', async ({ authPage: page }) => {
    await page.goto('/user/hardware');
    await expect(page).toHaveTitle('MediaKraken Hardware');
  });
});
