# Skynet llama.cpp distributed inference cluster

Two-node Docker Compose environment for running **Qwen3-Coder-Next-GGUF** (80B MoE, 3B active, 256K context) across a GPU host and a memory-rich host using llama.cpp's RPC backend.

```
                ┌──────────────────────────────────┐                ┌──────────────────────────────────┐
                │           skynetgpu1             │                │           skynetmem1             │
                │   RTX 5070 Ti  (16 GB VRAM)      │                │   128 GB RAM, 2 TB HDD           │
                │   CUDA 12.8 / sm_120 (Blackwell) │                │   CPU only                       │
                │                                  │                │                                  │
                │  ┌────────────────────────────┐  │   RPC / TCP    │  ┌────────────────────────────┐  │
                │  │  llama-server  :8080       │◄─┼────50052──────►│  │  rpc-server                │  │
                │  │   • OpenAI-compatible API  │  │                │  │   • CPU backend            │  │
                │  │   • attention + KV cache   │  │                │  │   • holds MoE experts      │  │
                │  │     pinned to GPU          │  │                │  │   • 100 GB backend pool    │  │
                │  └────────────────────────────┘  │                │  └────────────────────────────┘  │
                │              ▲                   │                │              ▲                   │
                │              │  reads GGUF       │     NFS        │              │   /srv/models    │
                │              └─── /models ◄──────┼─── (port ──────┼──► /models ──┘   (2 TB HDD)     │
                │                                  │     2049)      │                                  │
                │                                  │                │  ┌────────────────────────────┐  │
                │                                  │                │  │  nfs-server                │  │
                │                                  │                │  │   • exports /srv/models    │  │
                │                                  │                │  └────────────────────────────┘  │
                └──────────────────────────────────┘                └──────────────────────────────────┘
```

## Why this works

- **llama.cpp RPC** lets one `llama-server` process distribute model tensors across a local CUDA device plus one or more remote `rpc-server` instances. It's the upstream-supported way to do CPU+GPU sharded inference for GGUF.
- **Qwen3-Coder-Next** is a sparse MoE: only ~3B of 80B parameters activate per token. The expensive-to-store-but-rarely-active expert FFN tensors are the perfect thing to push to the RAM node, while the always-hot attention layers and KV cache stay on the GPU.
- The 5070 Ti is **Blackwell sm_120** — needs CUDA 12.8 and an llama.cpp build with `-DCMAKE_CUDA_ARCHITECTURES=120`. Pre-built `ghcr.io/ggml-org/llama.cpp:server-cuda` images don't reliably include sm_120 yet, so this stack builds from source.

## Layout

```
skynet-llama/
├── README.md                       this file
├── .env.example                    copy to .env on each node and edit
├── skynetgpu1/                     deploy on the GPU host
│   ├── docker-compose.yml
│   └── Dockerfile.llama-cuda
├── skynetmem1/                     deploy on the memory host
│   ├── docker-compose.yml
│   └── Dockerfile.llama-rpc
└── scripts/
    ├── download-model.sh           run once on skynetmem1
    ├── start-cluster.sh            convenience starter
    └── smoke-test.sh               curl /v1/chat/completions
```

## Prerequisites

| Node        | Required                                                                     |
| ----------- | ---------------------------------------------------------------------------- |
| both        | Docker Engine 27+, Docker Compose v2, network reachable between hosts        |
| skynetgpu1  | NVIDIA driver **570.86.10+**, NVIDIA Container Toolkit, CUDA 12.8 compatible |
| skynetmem1  | ≥ 110 GB free RAM, ≥ 100 GB free HDD for model + RPC cache                   |

Verify GPU on skynetgpu1:

```bash
nvidia-smi              # should show RTX 5070 Ti, driver ≥ 570
docker run --rm --gpus all nvidia/cuda:12.8.1-base-ubuntu24.04 nvidia-smi
```

## Setup

### 1. Configure `/etc/hosts` on **both** nodes

So containers can resolve peer hostnames without DNS:

```
192.168.1.10  skynetgpu1
192.168.1.20  skynetmem1
```

Replace IPs with your actual LAN addresses.

### 2. On `skynetmem1`: prepare storage and download the model

```bash
sudo mkdir -p /srv/models /srv/rpc-cache
sudo chown -R $USER:$USER /srv/models /srv/rpc-cache

cd skynet-llama
cp .env.example .env          # edit MODEL_QUANT if you want a different size
./scripts/download-model.sh
```

The default quant is `Q4_K_M` (~46 GB). Smaller options: `UD-Q3_K_XL` (~36 GB), `UD-Q2_K_XL` (~30 GB).

### 3. On `skynetmem1`: bring up the RPC + NFS services

```bash
cd skynet-llama/skynetmem1
docker compose up -d --build
docker compose logs -f rpc-server
```

You should see something like:

```
create_backend: using CPU backend
Starting RPC server v3.0.0
  endpoint       : 0.0.0.0:50052
  backend memory : 100000 MB
```

### 4. On `skynetgpu1`: mount the NFS share

This makes the GGUF on the mem node's HDD visible to the GPU container without copying ~46 GB.

```bash
sudo apt-get install -y nfs-common
sudo mkdir -p /mnt/skynet-models
sudo mount -t nfs -o ro,vers=4,nolock skynetmem1:/ /mnt/skynet-models
# add to /etc/fstab for persistence:
# skynetmem1:/  /mnt/skynet-models  nfs  ro,vers=4,nolock,_netdev  0 0
ls /mnt/skynet-models   # should list Qwen3-Coder-Next-*
```

### 5. On `skynetgpu1`: bring up llama-server

```bash
cd skynet-llama/skynetgpu1
cp ../.env.example .env       # set RPC_BACKEND=skynetmem1:50052, MODEL_FILE=...
docker compose up -d --build
docker compose logs -f llama-server
```

The first run builds llama.cpp from source against CUDA 12.8 with `sm_120` — takes ~15 min. Subsequent starts are instant.

Healthy output ends with:

```
srv  load_model: loading model from /models/Qwen3-Coder-Next-Q4_K_M/...gguf
llama_load_model_from_file: using device CUDA0 (NVIDIA GeForce RTX 5070 Ti)
llama_load_model_from_file: using device RPC[skynetmem1:50052]
...
main: HTTP server listening, port: 8080
```

### 6. Smoke test

From any machine that can reach skynetgpu1:8080:

```bash
./scripts/smoke-test.sh skynetgpu1
```

or directly:

```bash
curl http://skynetgpu1:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen3-Coder-Next",
    "messages": [{"role":"user","content":"Write a Python LRU cache decorator."}],
    "temperature": 0.7, "top_p": 0.8, "max_tokens": 512
  }'
```

## Tuning

All knobs live in the `command:` block of `skynetgpu1/docker-compose.yml`.

| Setting             | Default        | Notes                                                                                                   |
| ------------------- | -------------- | ------------------------------------------------------------------------------------------------------- |
| `--tensor-split`    | `16,100`       | Ratio of bytes to local CUDA vs RPC backend. Bump first number if VRAM headroom; bump second if OOM-CPU |
| `-ngl`              | `999`          | Offload all transformer blocks; the split decides where they actually land                              |
| `-c`                | `65536`        | Context window. 256K works but KV cache balloons — keep ≤ 64K unless you have a reason                  |
| `--cache-type-k/v`  | `q8_0`         | Quantized KV cache. Saves ~50% VRAM with negligible quality loss. `f16` for max quality                 |
| `-ot` override      | (commented)    | Force tensor placement by regex. Uncomment the MoE-experts-to-RPC line for explicit control             |
| `--threads` (GPU)   | `8`            | CPU threads for non-offloaded ops on the GPU host                                                       |
| `--threads` (mem)   | `16`           | rpc-server compute threads; set to physical core count of skynetmem1                                    |
| `-fa on`            | on             | Flash attention; required for long context                                                              |
| `--mem` (rpc)       | `100000` MB    | Backend memory pool. Stay ~15–20 GB below total RAM for OS + page cache                                 |

### Forcing MoE experts to the RAM node explicitly

The default `--tensor-split` does proportional distribution. For deterministic placement of expert FFNs, uncomment in `skynetgpu1/docker-compose.yml`:

```yaml
- -ot
- "\\.ffn_(gate|up|down)_exps\\.=RPC[skynetmem1:50052]"
```

This pins every MoE expert tensor to the RPC backend, leaving only attention + routing on the GPU. Best throughput-per-VRAM for this model.

## Troubleshooting

**`CUDA error: no kernel image is available for execution on the device`**
The image was built without sm_120. Rebuild with `docker compose build --no-cache` and confirm `CMAKE_CUDA_ARCHITECTURES=89;120` in the Dockerfile args.

**`failed to connect to skynetmem1:50052`**
Check `docker compose ps` on skynetmem1, then `nc -zv skynetmem1 50052` from skynetgpu1. The RPC port is **never** safe to expose to the internet — keep it on the LAN/VPN only.

**Tokens/sec is awful (< 5 tok/s)**
The bottleneck is almost always the network. RPC traffic for a 30B+ MoE on a 1 GbE link is painful. Use 10 GbE or direct cable between nodes. Also check `--tensor-split` — too much on GPU forces excessive RPC round-trips per token.

**`rpc-server` OOMs**
Lower `--mem`, or drop to a smaller quant (`UD-Q3_K_XL` → `UD-Q2_K_XL`). The `--mem` value is a hard cap, not a target.

**`invalid resource handle` on the GPU**
A known Blackwell + custom ggml kernel issue. Rebuild the GPU image with the cuBLAS-forced variant by setting `GGML_CUDA_FORCE_CUBLAS=ON` in the Dockerfile (already on by default here).

## Security

RPC has **no authentication**. Treat port 50052 like an internal database port — firewall it to the GPU node's IP only. The NFS export is read-only but should also be LAN-restricted.

## References

- llama.cpp RPC docs: <https://github.com/ggml-org/llama.cpp/blob/master/tools/rpc/README.md>
- Qwen3-Coder-Next model card: <https://huggingface.co/Qwen/Qwen3-Coder-Next-GGUF>
- Unsloth Qwen3-Coder-Next guide: <https://unsloth.ai/docs/models/qwen3-coder-next>
