#!/usr/bin/env python3
"""Stream a WAV file to a Vosk WebSocket server and print recognised text."""

import argparse
import asyncio
import json
import wave

import websockets


async def stream(wav_path: str, uri: str) -> None:
    with wave.open(wav_path, "rb") as wf:
        buffer_size = int(wf.getframerate() * 0.2)  # 0.2 seconds per chunk
        async with websockets.connect(uri) as websocket:
            await websocket.send(
                json.dumps({"config": {"sample_rate": wf.getframerate()}})
            )
            while True:
                data = wf.readframes(buffer_size)
                if not data:
                    break
                await websocket.send(data)
                print(await websocket.recv())
            await websocket.send(json.dumps({"eof": 1}))
            print(await websocket.recv())


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "wav",
        nargs="?",
        default="voice_file_mono.wav",
        help="WAV file to stream (default: voice_file_mono.wav)",
    )
    parser.add_argument(
        "uri",
        nargs="?",
        default="ws://mkprod:2700",
        help="Vosk WebSocket URI (default: ws://mkprod:2700)",
    )
    args = parser.parse_args()
    asyncio.run(stream(args.wav, args.uri))


if __name__ == "__main__":
    main()
