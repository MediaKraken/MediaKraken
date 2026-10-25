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

### 2. Android SDK setup

Two toolchains are involved:

- `dx` compiles the Rust code with the **NDK** clang toolchain
  (needs `ANDROID_HOME` and `NDK_HOME`).
- The generated gradle project (AGP 8.7.0, Kotlin 2.0.20, `compileSdk 34`,
  `minSdk 24`, `targetSdk 34`) assembles the APK and requires **JDK 17**.

#### Option A — Android Studio (GUI)

1. Install [Android Studio](https://developer.android.com/studio); it creates
   the SDK at `~/Android/Sdk` (Linux) or `~/Library/Android/sdk` (macOS).
2. *SDK Manager → SDK Platforms*: install **Android API 34 (Android 14)**.
   *SDK Manager → SDK Tools*: enable **Android SDK Platform-Tools**,
   **Android SDK Build-Tools**, **NDK (Side by side)**, **CMake**, and
   **Android SDK Command-line Tools (latest)**; accept all licenses.
3. *Device Manager*: create at least one emulator (AVD) with an API 34
   system image.

#### Option B — command line only (Linux / macOS, no Studio)

```bash
# 1. SDK command-line tools
sudo apt install -y unzip                      # Debian / Ubuntu
mkdir -p "$HOME/android-sdk/cmdline-tools"
curl -O https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip
unzip commandlinetools-linux-11076708_latest.zip -d "$HOME/android-sdk/cmdline-tools"
mv "$HOME/android-sdk/cmdline-tools/cmdline-tools" "$HOME/android-sdk/cmdline-tools/latest"

# 2. Packages dx needs (API 34 matches compileSdk 34 of the generated project)
SDKMGR="$HOME/android-sdk/cmdline-tools/latest/bin/sdkmanager"
"$SDKMGR" --install \
  "platform-tools" \
  "platforms;android-34" \
  "build-tools;34.0.0" \
  "ndk;27.2.12479018" \
  "emulator" \
  "system-images;android-34;google_apis;x86_64"
yes | "$SDKMGR" --licenses

# 3. JDK 17 — required by AGP 8.7.0
sudo apt install -y openjdk-17-jdk             # Debian / Ubuntu
```

- Any recent NDK works — `dx` automatically uses the newest directory under
  `$ANDROID_HOME/ndk/`; list alternatives with `sdkmanager --list`.
- On an arm64 Linux host (e.g. Raspberry Pi 5) install
  `system-images;android-34;google_apis;arm64-v8a` instead of `x86_64`.
- On macOS use the `commandlinetools-mac-…` archive from the
  [Command-line tools](https://developer.android.com/studio#command-line-tools-only)
  section of the official docs.

#### Environment

```bash
export ANDROID_HOME="$HOME/Android/Sdk"                  # or $HOME/android-sdk, or $HOME/Library/Android/sdk
export JAVA_HOME="/usr/lib/jvm/java-17-openjdk-amd64"    # AGP 8.7.0 needs JDK 17; Android Studio's JBR also works
export NDK_HOME="$ANDROID_HOME/ndk/27.2.12479018"        # optional — dx auto-detects the newest NDK
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

Add these to your `~/.bashrc` / `~/.zshrc` to make them permanent. `dx`
resolves the SDK from `ANDROID_SDK_ROOT`, `ANDROID_SDK`, or `ANDROID_HOME`,
and the NDK from `NDK_HOME` / `ANDROID_NDK_HOME` (falling back to the newest
`$ANDROID_HOME/ndk/<version>` directory).

#### Create an emulator (if you don't have one yet)

```bash
avdmanager create avd -n pixel_6 -d pixel_6 \
  -k "system-images;android-34;google_apis;x86_64"
emulator -avd pixel_6 &          # start it; dx serve picks it up automatically
```

#### Verify

```bash
adb devices      # the running emulator (or a USB-connected phone) is listed
dx doctor        # under "Android": sdk, ndk, adb, emulator, java_home are all found
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
