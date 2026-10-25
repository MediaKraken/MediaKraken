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

Mobile builds are driven by the Dioxus CLI (`dx`), which wraps the Rust app in a
native harness (Android gradle project / iOS Xcode project). Use a `dioxus-cli`
0.7.x release matching the `dioxus = "0.7.10"` dependency in `Cargo.toml`.

Run all commands from the package directory:

```bash
cd src_app/mediakraken
```

`dx` detects the enclosing `src_app` workspace and builds the `mediakraken`
package.

### 1. Install the Dioxus CLI and Rust targets

```bash
# Dioxus CLI (0.7.x)
curl -sSL https://dioxus.dev/install.sh | bash
# or: cargo binstall dioxus-cli --force

# Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# iOS targets (only needed on macOS)
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

Verify the environment with:

```bash
dx doctor
```

### 2. Android prerequisites (Linux, macOS or Windows)

- Install [Android Studio](https://developer.android.com/studio).
- In *SDK Manager → SDK Tools* install: Android SDK, SDK Command-line tools,
  NDK (Side by side), and CMake.
- Create at least one emulator (AVD) in the *Device Manager*.
- Export the environment (adjust paths to your installation):

```bash
export JAVA_HOME="/path/to/android-studio/jbr"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<installed-ndk-version>"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

### 3. iOS prerequisites (macOS only)

- Install [Xcode](https://apps.apple.com/gb/app/xcode/id497799835) and accept
  the licenses.
- Install the iOS SDK and at least one simulator
  (Xcode → *Settings → Platforms / Components*).
- An Apple Developer account is only needed to sign for a physical device or
  the App Store; the simulator works without one.

### 4. Develop (hot reload)

Start the matching emulator/simulator first, then:

```bash
# Android (e.g. emulator -avd Pixel_6_API_34)
dx serve --android

# iOS (e.g. open -a Simulator && xcrun simctl boot "iPhone 15 Pro Max")
dx serve --ios
```

`dx` builds the app, installs it on the active device, and hot-reloads on
changes. Use `--device <name>` to target a specific device instead.

### 5. Release builds

```bash
# Android: .aab (Play Store default) or .apk
dx bundle --android --release
dx bundle --android --release --package-types apk

# iOS: .ipa (device target aarch64-apple-ios) or .app
dx bundle --ios --release
dx bundle --ios --release --package-types ios
```

`dx` prints the paths of the produced artifacts on success (by default under
`src_app/target/dx/mediakraken/...`).

### 6. `Dioxus.toml` (app metadata, permissions, signing)

The checked-in `src_app/mediakraken/Dioxus.toml` provides the app metadata
and the camera permission for the Scan page (plus a commented-out Android
keystore block, see below):

```toml
[application]
name = "mediakraken"

[bundle]
identifier = "media.mediakraken"   # Android applicationId / iOS bundle id
publisher = "MediaKraken"

# The Scan page uses the device camera for UPC scanning
[permissions.camera]
description = "Scan media barcodes"
```

If you want a different app ID, change `identifier` (and the matching Play
Store package name / iOS bundle id) before distributing.

- **Android signing**: `--release` builds are only signed when a keystore is
  configured:

  ```toml
  [bundle.android]
  jks_file = "keystore/release.jks"
  jks_password = "REDACTED"
  key_alias = "release"
  key_password = "REDACTED"
  ```

  Generate the keystore with
  `keytool -genkey -v -keystore release.jks -keyalg RSA -keysize 2048 -validity 10000`.
  The values are read verbatim from `Dioxus.toml` — do not commit the keystore
  file or real passwords.

- **iOS signing**: pass the signing options to the bundle command:

  ```bash
  dx bundle --ios --release \
      --codesign \
      --apple_team_id "Your Name (TEAMID123)" \
      --apple_entitlements ios/mediakraken.entitlements
  ```

  Device builds and App Store distribution require an Apple Developer account
  and a provisioning profile; see the
  [Tauri signing guide](https://tauri.app/distribute/sign/iOS/) for the full
  process (`dx` uses the same `codesign`/`xcrun` tooling).

### Notes

- iOS builds require a macOS host; Android builds work on any host OS.
- The generated gradle/Xcode project is created inside the `src_app/target/`
  build area — no `android/` or `ios/` source directories are added to the
  repo.
- After installing, point the app at a reachable MediaKraken API base URL via
  the config form in the app header (defaults to
  `https://www.mediakraken.media`).
