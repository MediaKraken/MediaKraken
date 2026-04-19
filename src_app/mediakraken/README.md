# MediaKraken Dioxus mobile boilerplate

This crate is a minimal Dioxus starter app intended for iOS and Android clients that call the MediaKraken HTTP API.

# to build desktop
sudo apt install -y libxdo-dev
apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev

## What is included

- A small `ApiClient` wrapper around `reqwest`
- A simple login/config form for the MediaKraken API base URL and bearer token
- Tab-based navigation between Home, Scan, Hardware, and Media pages
- Conditional launch targets for Dioxus mobile (`ios` / `android`) and desktop fallback for local UI development

## Pages

- **Home** — Calls `GET /api/v1/library/summary` to verify connectivity.
- **Scan** — Opens the device camera (`<input capture="environment">`) and posts
  the captured UPC to `GET /api/v1/library/ownership?upc=<code>` to check whether
  the title is already in the user's library.
- **Hardware** — Lists devices from `GET /api/v1/hardware/devices` and exposes
  power/mute toggles plus a volume slider that POST to
  `/api/v1/hardware/devices/{id}/{power|mute|volume}`.
- **Media** — Browses movies, TV shows, and audio via
  `GET /api/v1/media/{movies|shows|audio}` and renders a poster grid.

## Expected API contracts

```json
// /api/v1/library/summary
{ "server_name": "MediaKraken", "version": "0.0.1",
  "movie_count": 120, "show_count": 45, "music_album_count": 12 }

// /api/v1/library/ownership?upc=786936224290
{ "upc": "786936224290", "owned": true,
  "title": "WALL·E", "media_type": "movie", "year": 2008 }

// /api/v1/hardware/devices
{ "devices": [
    { "id": "living-room-avr", "name": "Living Room AVR",
      "kind": "receiver", "powered": true, "volume": 42, "muted": false }
] }

// /api/v1/media/movies | /shows | /audio
{ "items": [
    { "id": "abc123", "title": "Example", "year": 2024,
      "media_type": "movie", "poster_url": "https://...",
      "overview": "..." }
] }
```

If your API endpoint differs, update `src/api.rs` and `src/models.rs`.

## Local desktop preview

From the `src_app/` workspace root:

```bash
cargo run -p mediakraken
```

## Android / iOS

Use the Dioxus mobile tooling that matches the installed SDKs in your environment, then point the app at a reachable MediaKraken API base URL.
