# MediaKraken Dioxus mobile boilerplate

This crate is a minimal Dioxus starter app intended for iOS and Android clients that call the MediaKraken HTTP API.

## What is included

- A small `ApiClient` wrapper around `reqwest`
- A simple login/config form for the MediaKraken API base URL and bearer token
- A sample `GET /api/v1/library/summary` request path for initial integration work
- Conditional launch targets for Dioxus mobile (`ios` / `android`) and desktop fallback for local UI development

## Expected API contract

The boilerplate expects a JSON response shaped roughly like this:

```json
{
  "server_name": "MediaKraken",
  "version": "0.0.1",
  "movie_count": 120,
  "show_count": 45,
  "music_album_count": 12
}
```

If your API endpoint differs, update `src/api.rs` and `src/models.rs`.

## Local desktop preview

From the `src_app/` workspace root:

```bash
cargo run -p mediakraken
```

### Linux system dependencies

The Dioxus `desktop` feature links against `libxdo` (pulled in transitively via
`muda`/`tray-icon`). If the linker reports
`rust-lld: error: unable to find library -lxdo`, install the matching system
package before rebuilding:

- Debian / Ubuntu: `sudo apt install libxdo-dev`
- Fedora / RHEL: `sudo dnf install libxdo-devel`
- Arch: `sudo pacman -S xdotool`

GTK and WebKit2GTK development headers are also required by Dioxus desktop on
Linux (`libgtk-3-dev`, `libwebkit2gtk-4.1-dev` on Debian/Ubuntu).

## Android / iOS

Use the Dioxus mobile tooling that matches the installed SDKs in your environment, then point the app at a reachable MediaKraken API base URL.
