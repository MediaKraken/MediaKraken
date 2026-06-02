debian 13
apt install firmware-atheros firmware-realtek network-manager
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


apt install linux-headers-$(uname -r) build-essential

wget https://developer.download.nvidia.com/compute/cuda/repos/debian13/x86_64/cuda-keyring_1.1-1_all.deb
dpkg -i cuda-keyring_1.1-1_all.deb
apt-get update

apt install cuda

reboot


apt install nvidia-driver firmware-misc-nonfree xauth nvtop python3-pip
reboot


mokutil --disable-validation
reboot and be at keyboard!!!!!!!
disable

nvidia-smi


nano /etc/ssh/sshd_config

X11Forwarding yes
X11UseLocalhost yes

systemctl restart ssh

************************************************
apt install curl libcurl4-openssl-dev

curl -fsSL https://unsloth.ai/install.sh | sh


~/.local/bin/unsloth studio -H 192.168.1.157 -p 8888

http://localai.mediakraken.media:8888

unsloth studio update

*********************************************

apt install nodejs npm git gh -y
npm install -g @openai/codex
###codex upgrade - rerun the command above

mkdir ~/.codex ~/.codex/model-catalogs
nano ~/.codex/config.toml

personality = "friendly"
profile     = "unsloth_api9b"

[model_providers.unsloth_api]
name                  = "Unsloth Studio"
base_url              = "http://192.168.1.157:8888/v1"
env_key               = "UNSLOTH_STUDIO_AUTH_TOKEN"
wire_api              = "responses"
requires_openai_auth  = false

[profiles.unsloth_api27b]
model_provider = "unsloth_api"
model          = "unsloth/Qwen3.6-27B-GGUF"
model_catalog_json = "~/.codex/model-catalogs/qwen36.json"

[profiles.unsloth_api9b]
model_provider = "unsloth_api"
model          = "unsloth/Qwen3.5-9B-GGUF"
model_catalog_json = "~/.codex/model-catalogs/qwen359b.json"

[projects."/home/spoot/my-project"]
trust_level = "trusted"

[features]
collaboration_modes = true
steer = true
unified_exec = true
apps = false



nano ~/.codex/model-catalogs/qwen36.json

{
  "models": [
    {
      "slug": "unsloth/Qwen3.6-27B-GGUF",
      "display_name": "Qwen3.6-27B",
      "description": "Alibaba Qwen3.6-27B dense model. Agentic coding, repository-level reasoning, and built-in thinking mode. Served via an OpenAI-compatible endpoint (vLLM, SGLang, Ollama, or LM Studio).",
      "context_window": 262144,
      "max_context_window": 1010000,
      "auto_compact_token_limit": null,
      "input_modalities": [
        "text"
      ],
      "supports_image_detail_original": false,
      "truncation_policy": {
        "mode": "tokens",
        "limit": 10000
      },
      "supports_parallel_tool_calls": true,
      "apply_patch_tool_type": "freeform",
      "web_search_tool_type": "text",
      "supports_search_tool": false,
      "supports_reasoning_summaries": false,
      "reasoning_summary_format": "none",
      "default_reasoning_summary": "none",
      "default_reasoning_level": "medium",
      "supported_reasoning_levels": [
        {
          "effort": "low",
          "description": "Fast responses with minimal thinking"
        },
        {
          "effort": "medium",
          "description": "Balanced thinking depth for everyday tasks"
        },
        {
          "effort": "high",
          "description": "Extended thinking for complex coding and reasoning"
        }
      ],
      "prefer_websockets": false,
      "support_verbosity": false,
      "default_verbosity": "low",
      "shell_type": "shell_command",
      "visibility": "list",
      "minimal_client_version": "0.130.0",
      "supported_in_api": true,
      "availability_nux": null,
      "upgrade": null,
      "priority": 50,
      "experimental_supported_tools": [],
      "available_in_plans": [],
      "additional_speed_tiers": [],
      "base_instructions": "You are Codex, a coding agent running on Qwen3.6-27B. You and the user share one workspace; collaborate with them until the task is genuinely handled.\n\n# General\n- Read the codebase before making assumptions. Build context from real files, not memory.\n- Prefer `rg` and `rg --files` for searches; fall back to `grep`/`find` only when ripgrep is unavailable.\n- Parallelize independent read-only tool calls (cat, rg, ls, sed, nl, wc, git show). Never chain shell commands with `;` separators just to inspect output.\n- Use `apply_patch` for all manual code edits. Do not write files with `cat <<EOF` or `echo >`.\n- Default to ASCII unless the surrounding file already uses Unicode.\n- Add short comments only where code is not self-explanatory.\n\n# Editing constraints\n- Never revert changes you did not make. If you see unrelated edits in the worktree, leave them alone.\n- Never run destructive git commands (`git reset --hard`, `git checkout --`, force pushes) without explicit user approval.\n- Prefer non-interactive git invocations.\n\n# Autonomy\n- Persist until the task is complete: implement, verify, and summarize. Do not stop at a proposal unless the user only asked for analysis or a plan.\n- If you hit a blocker, attempt to resolve it before handing the problem back.\n\n# Reasoning\n- Use the thinking mode for non-trivial problems; keep low-effort responses for trivial edits and chit-chat.\n- When the user asks for a review, lead with findings ordered by severity (with file:line references), then summarize.\n\n# Output formatting\n- GitHub-flavored Markdown is fine.\n- Flat lists only; no nested bullets. Numbered lists use `1. 2. 3.`.\n- Wrap commands, paths, env vars, and identifiers in backticks; use fenced code blocks for multi-line snippets with an info string.\n- File references should be markdown links with absolute paths, e.g. `[app.py](/abs/path/app.py:42)`.\n- No emojis or em dashes unless the user uses them first.\n- Keep final answers concise; for simple tasks, one or two short paragraphs are usually enough.\n"
    }
  ]
}




{
  "models": [
    {
      "id": "unsloth/Qwen3.5-9B-GGUF:UD-Q4_K_XL",
      "slug": "qwen35-9b-unsloth",
      "name": "Qwen 3.5 9B (Unsloth GGUF)",
      "provider": "oss",
      "max_context_window": 32768,
      "max_output_tokens": 8192,
      "capabilities": {
        "chat": true,
        "completion": true,
        "tools": true,
        "function_calling": false,
        "structured_output": true,
        "inline_thinking": true
      },
      "pricing": {
        "input_cost_per_1k": 0.0,
        "output_cost_per_1k": 0.0
      },
      "stop_sequences": [
        "<|im_end|>",
        "<|im_start|>"
      ]
    }
  ]
}



export UNSLOTH_STUDIO_AUTH_TOKEN=sk-unsloth-2d8cf20cf075c80a616de77c6ea2d365




mkdir my-project && cd my-project
codex -p unsloth_api


**************************

codexmonitor
wget https://github.com/Dimillian/CodexMonitor/releases/download/v0.7.67/Codex.Monitor_0.7.67_amd64.AppImage


vscode plugin
Continue - open-source AI code agent


**********************************
docker/compose

nano docker-compose.yml
services:
  openwebui:
    image: ghcr.io/open-webui/open-webui:cuda
    ports:
      - "3000:8080"
    volumes:
      - open-webui:/app/backend/data
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]

volumes:
  open-webui:

# 1. Configure the production repository
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

# 2. Update and install
apt-get update
apt-get install -y nvidia-container-toolkit


# 1. Register the NVIDIA runtime with Docker's configuration
nvidia-ctk runtime configure --runtime=docker

# 2. Restart the Docker engine to apply the changes
systemctl restart docker

docker run --rm --gpus all nvidia/cuda:12.0.0-base-ubuntu22.04 nvidia-smi

https://192.168.1.157:3000

***************************
open webui - lol no, wants 3.11
apt install python3-pip
pip3 install open-webui


********************************

curl -fsSL https://opencode.ai/install | bash

opencode
/connect


nano ~/.config/opencode/opencode.json

{
  "$schema": "https://opencode.ai/config.json",
  "model": "llama-cpp/qwen36-35b-a3b-ud-q4_k_xl",
  "provider": {
    "llama-cpp": {
      "npm": "@ai-sdk/openai-compatible",
      "name": "llama.cpp (skynetgpu1)",
      "options": {
        "baseURL": "http://skynetgpu1.mediakraken.media:8000/v1",
        "apiKey": "sk-no-key-required"
      },
      "models": {
        "qwen36-35b-a3b-ud-q4_k_xl": {
          "name": "qwen36-35b-a3b-ud-q4_k_xl",
          "tool_call": true,
          "limit": {
            "context": 131072,
            "output": 32768
          }
        }
      }
    }
  }
}

/model then select skynet
