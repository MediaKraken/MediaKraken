#!/usr/bin/env bash
# =============================================================================
# Smoke test the cluster end-to-end.
#   ./smoke-test.sh            # defaults to localhost
#   ./smoke-test.sh skynetgpu1
# =============================================================================
set -euo pipefail

HOST="${1:-localhost}"
PORT="${2:-8080}"
BASE="http://${HOST}:${PORT}"

echo "▶ Health check"
curl -fsS "${BASE}/health" && echo

echo
echo "▶ Model info"
curl -fsS "${BASE}/v1/models" | python3 -m json.tool

echo
echo "▶ Chat completion"
curl -fsS "${BASE}/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "Qwen3-Coder-Next",
        "messages": [
            {"role": "system", "content": "You are a senior Python engineer."},
            {"role": "user",   "content": "Write a thread-safe LRU cache decorator in Python 3.12. No external deps. Include a docstring and one usage example."}
        ],
        "temperature": 0.7,
        "top_p": 0.8,
        "top_k": 20,
        "max_tokens": 800,
        "stream": false
    }' | python3 -c '
import json, sys
r = json.load(sys.stdin)
msg = r["choices"][0]["message"]["content"]
usage = r.get("usage", {})
print("─── response ───")
print(msg)
print("─── usage ───")
print(json.dumps(usage, indent=2))
'
