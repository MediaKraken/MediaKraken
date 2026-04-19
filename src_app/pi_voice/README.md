# pi_voice

Small FLTK-based client that records audio, ships it to a Vosk
WebSocket server for speech recognition, and uses the recognised text
to drive a MediaKraken title search in an embedded webview.

## System dependencies

- `libasound2-dev` (ALSA headers, for `cpal`)
- `libwebkit2gtk-4.1-dev` (for `fltk-webview`)
- `ffmpeg` on `$PATH` (to resample the recording to 16 kHz mono)
- `python3` with the packages in `requirements.txt`
- A reachable Vosk WebSocket server

## Build

```
cargo build --release
```

## Python helper

```
python3 -m pip install -r requirements.txt
```

## Run

```
./pi_voice \
    --api-base https://mkprod:8900 \
    --vosk-uri ws://mkprod:2700 \
    --workdir /path/to/pi_voice
```

All flags are optional. `--workdir` should contain
`send_wav_to_websocket.py`; it defaults to the directory of the
executable.

## UI

- **Start Record** – begin capturing from the default input device
- **Stop Record** – finalise the WAV file
- **Recognise** – stop recording (if active), convert via ffmpeg,
  stream to Vosk, and navigate the embedded webview to the matching
  title-search page. The media-type dropdown is attached to the
  request as `?media_type=...`.

Recognition runs on a worker thread so the UI stays responsive.
