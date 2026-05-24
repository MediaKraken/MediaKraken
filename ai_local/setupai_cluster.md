# Prompt
Node 1 (skynetgpu1): Run inference on Qwen3-Coder-Next-GGUF using GPU (RTX 5070 Ti).
Node 2 (skynetmem1): Provide extra memory (RAM/HDD) — e.g., offload KV-cache or serve as a memory pool (128GB RAM, 2TB HDD).
Since Qwen3-Coder-Next-GGUF is a quantized model, and you want to leverage distributed memory, the best approach is:

Use vLLM’s --distributed-executor-backend ray with Ray + multi-node setup, where:

skynetgpu1 runs the GPU worker (vLLM inference),
skynetmem1 runs a Ray worker node that can store offloaded tensors (KV-cache, weights, etc.) in its large RAM.


# Operating system build
Debian 13
apt install ca-certificates curl gnupg lsb-release libfuse2 python3 python3-pip -y

install -m 0755 -d /etc/apt/keyrings && \
curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc && \
chmod a+r /etc/apt/keyrings/docker.asc

## Add the repository to Apt sources:
tee /etc/apt/sources.list.d/docker.sources <<EOF
Types: deb
URIs: https://download.docker.com/linux/debian
Suites: $(. /etc/os-release && echo "$VERSION_CODENAME")
Components: stable
Architectures: $(dpkg --print-architecture)
Signed-By: /etc/apt/keyrings/docker.asc
EOF

apt update && apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y


# Setup CUDA on nodes with GPU
was done on gpu1



# On skynetmem1
mkdir -p /data/gguf && cd /data/gguf
# Use hf-transfer or direct download (e.g., via HuggingFace CLI)
pip3 install huggingface-hub --break-system-packages
hf download unsloth/Qwen3-Coder-Next-GGUF --include "Qwen3-Coder-Next-UD-TQ1_0.gguf" --local-dir ./qwen3-coder-next-ud-tq1_0
then move the gguf file to /data/gguf/.


# On skynetgpu1
mkdir -p /data/gguf && cd /data/gguf
scp spoot@skynetmem1://data/gguf/Qwen3-Coder-Next-UD-TQ1_0.gguf .











*************************************

Ensure skynetgpu1 and skynetmem1 can reach each other via TCP (ports 6379, 8265, 29500, etc.).


***************************************

Test cluster
# Basic validation
python validate_kv_cache.py

# With custom settings
python validate_kv_cache.py --host my-kv-cache.com --port 6379 --timeout 10

# Summary mode only
python validate_kv_cache.py --summary



Test inference
curl -X POST -H "Content-Type: application/json" \
  -d '{"prompt": "Explain quantum computing.", "max_tokens": 512}' \
  http://skynetgpu1:8000/generate

Ray Dashboard
http://skynetgpu1:8265