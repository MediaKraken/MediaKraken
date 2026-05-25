apt install jq -y

mkdir -p /data/gguf
cd /data/gguf
wget https://huggingface.co/unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF/resolve/main/Qwen3-Coder-30B-A3B-Instruct-Q4_K_M.gguf



this is the docker on gpu1
Tune --n-cpu-moe: start at 20, if you OOM bump to 25-30, if you have VRAM to spare lower to 15.

on mem1
wget https://huggingface.co/Qwen/Qwen3-Embedding-0.6B-GGUF/resolve/main/Qwen3-Embedding-0.6B-Q8_0.gguf


On skynetgpu1: docker compose up -d, then curl http://skynetgpu1:8000/v1/models — should show qwen3-coder.
On skynetmem1: docker compose up -d, then curl http://skynetmem1:6333/collections and curl http://skynetmem1:8001/v1/models

Aider config on your dev box
nano ~/.aider.conf.yml



Save as index_repo.py on whichever machine has the source code:
pip3 install qdrant-client httpx --break-system-packages
pip3 install aider-install --break-system-packages && aider-install
python3 ai_local/aider/index_repo.py ~/MediaKraken
    let the above command cook.....it'll be awhile

Aider run from inside your repo:
cd ~/MediaKraken
aider







On dev box: run index_repo.py against your repo.
On dev box: cd repo && aider. Ask it something repo-wide like "summarize how authentication flows through this codebase" — if it answers coherently using files it found via the repo map, you're set.



To query Qdrant from the shell while you wait for an integration:
curl http://skynetmem1:8001/v1/embeddings \
  -H "Content-Type: application/json" \
  -d '{"model":"BAAI/bge-m3","input":"how does auth middleware work"}' \
  | jq '.data[0].embedding' \
  | curl http://skynetmem1:6333/collections/code/points/search \
    -X POST -H "Content-Type: application/json" \
    -d @- --data-binary '{"vector":'"$(cat)"',"limit":10,"with_payload":true}'
