#!/usr/bin/env bash
# =============================================================================
# Download Qwen3-Coder-Next-GGUF to $MODELS_DIR on skynetmem1.
# Run once, on skynetmem1.
# =============================================================================
set -euo pipefail

# Load .env from the project root (one level up from this script).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
if [[ -f "$ROOT_DIR/.env" ]]; then
    set -a; source "$ROOT_DIR/.env"; set +a
fi

MODELS_DIR="${MODELS_DIR:-/srv/models}"
MODEL_REPO="${MODEL_REPO:-unsloth/Qwen3-Coder-Next-GGUF}"
MODEL_QUANT="${MODEL_QUANT:-Q4_K_M}"

echo "================================================================"
echo " Downloading $MODEL_REPO ($MODEL_QUANT)"
echo " Destination: $MODELS_DIR"
echo "================================================================"

# huggingface_hub is the official CLI. Install in a venv to keep system clean.
if ! command -v huggingface-cli >/dev/null 2>&1; then
    echo "Installing huggingface_hub[cli]..."
    if ! command -v pipx >/dev/null 2>&1; then
        sudo apt-get update && sudo apt-get install -y pipx
    fi
    pipx install "huggingface_hub[cli]"
    pipx ensurepath
    # shellcheck disable=SC1090
    source ~/.bashrc || true
fi

# hf_transfer gives a ~5x speedup on large multi-shard downloads.
pip install --user --quiet hf_transfer || true
export HF_HUB_ENABLE_HF_TRANSFER=1

mkdir -p "$MODELS_DIR"

echo
echo "Downloading shards matching pattern: *${MODEL_QUANT}*"
huggingface-cli download \
    "$MODEL_REPO" \
    --include "*${MODEL_QUANT}*" \
    --local-dir "$MODELS_DIR" \
    --local-dir-use-symlinks False

echo
echo "Done. Files:"
find "$MODELS_DIR" -name "*${MODEL_QUANT}*.gguf" -printf "  %p  (%s bytes)\n"

cat <<EOF

================================================================
 Next steps:
   1. On skynetmem1:  cd skynet-llama/skynetmem1 && docker compose up -d --build
   2. On skynetgpu1:  mount the NFS share, then:
                      cd skynet-llama/skynetgpu1 && docker compose up -d --build
================================================================
EOF
