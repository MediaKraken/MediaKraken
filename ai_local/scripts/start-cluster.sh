#!/usr/bin/env bash
# =============================================================================
# Bring up the local node's services. Detects which node we're on by hostname.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
HOST="$(hostname -s)"

case "$HOST" in
    skynetmem1)
        echo "[$HOST] Starting RPC + NFS services..."
        cd "$ROOT_DIR/skynetmem1"
        docker compose up -d --build
        echo
        echo "Tail logs with:  docker compose logs -f rpc-server"
        ;;
    skynetgpu1)
        echo "[$HOST] Starting llama-server (CUDA)..."
        # Sanity check: NFS share must already be mounted on the host.
        GPU_MODELS_DIR="${GPU_MODELS_DIR:-/mnt/skynet-models}"
        if ! mountpoint -q "$GPU_MODELS_DIR"; then
            echo "ERROR: $GPU_MODELS_DIR is not a mountpoint."
            echo "  Mount the NFS share first:"
            echo "    sudo mount -t nfs -o ro,vers=4,nolock \\"
            echo "      skynetmem1:/ $GPU_MODELS_DIR"
            exit 1
        fi
        cd "$ROOT_DIR/skynetgpu1"
        docker compose up -d --build
        echo
        echo "Tail logs with:  docker compose logs -f llama-server"
        echo "Endpoint:        http://$(hostname -f):8080"
        ;;
    *)
        echo "Unknown hostname '$HOST'. Expected skynetgpu1 or skynetmem1."
        echo "Set the hostname or run docker compose manually from the right dir."
        exit 1
        ;;
esac
