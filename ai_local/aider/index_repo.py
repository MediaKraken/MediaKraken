#!/usr/bin/env python3
"""Index a code repo into Qdrant for semantic search."""
import os, sys, hashlib, fnmatch, subprocess
from pathlib import Path
import httpx
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct
import time

QDRANT_URL = "http://skynetmem1:6333"
EMBED_URL = "http://skynetgpu1:8001/v1/embeddings"
EMBED_MODEL = "BAAI/bge-m3"
COLLECTION = "code"
VECTOR_SIZE = 1024
CHUNK_LINES = 60
CHUNK_OVERLAP = 10

CODE_EXTS = {".py",".js",".ts",".tsx",".jsx",".go",".rs",".java",".c",".cc",
             ".cpp",".h",".hpp",".rb",".php",".cs",".kt",".swift",".scala",
             ".sh",".yaml",".yml",".toml",".sql",".md"}

def gitignored(repo_root):
    """Return list of paths to skip, using git itself."""
    try:
        out = subprocess.check_output(
            ["git","-C",repo_root,"ls-files","-o","-i","--exclude-standard"],
            text=True
        )
        return set(out.splitlines())
    except subprocess.CalledProcessError:
        return set()

MAX_CHARS = 4000   # ~1000 tokens, well under context

def chunk_file(path: Path):
    try:
        lines = path.read_text(errors="ignore").splitlines()
    except Exception:
        return
    i = 0
    while i < len(lines):
        block = lines[i:i+CHUNK_LINES]
        if any(l.strip() for l in block):
            text = "\n".join(block)[:MAX_CHARS]
            yield i+1, i+len(block), text
        i += CHUNK_LINES - CHUNK_OVERLAP

def embed(texts, retries=3):
    for attempt in range(retries):
        try:
            r = httpx.post(EMBED_URL, json={"model": EMBED_MODEL, "input": texts}, timeout=600)
            r.raise_for_status()
            return [d["embedding"] for d in r.json()["data"]]
        except (httpx.ReadTimeout, httpx.HTTPStatusError) as e:
            if attempt == retries - 1:
                raise
            wait = 5 * (attempt + 1)
            print(f"  embed failed ({e}), retrying in {wait}s...")
            time.sleep(wait)

def main(repo_root):
    repo_root = Path(repo_root).resolve()
    qc = QdrantClient(url=QDRANT_URL)
    qc.recreate_collection(
        COLLECTION,
        vectors_config=VectorParams(size=VECTOR_SIZE, distance=Distance.COSINE),
    )
    ignored = gitignored(str(repo_root))

    batch_texts, batch_meta = [], []
    points = []
    point_id = 0

    for root, dirs, files in os.walk(repo_root):
        dirs[:] = [d for d in dirs if d not in (".git","node_modules","__pycache__",".venv","dist","build","target")]
        for fname in files:
            p = Path(root)/fname
            rel = str(p.relative_to(repo_root))
            if rel in ignored: continue
            if p.suffix.lower() not in CODE_EXTS: continue
            if p.stat().st_size > 500_000: continue
            for start, end, text in chunk_file(p):
                batch_texts.append(text)
                batch_meta.append({"path": rel, "start": start, "end": end})
                if len(batch_texts) >= 8:
                    vecs = embed(batch_texts)
                    for v, m in zip(vecs, batch_meta):
                        points.append(PointStruct(id=point_id, vector=v, payload=m))
                        point_id += 1
                    batch_texts, batch_meta = [], []
                    if len(points) >= 256:
                        qc.upsert(COLLECTION, points=points)
                        print(f"  indexed {point_id} chunks...")
                        points = []

    if batch_texts:
        vecs = embed(batch_texts)
        for v, m in zip(vecs, batch_meta):
            points.append(PointStruct(id=point_id, vector=v, payload=m))
            point_id += 1
    if points:
        qc.upsert(COLLECTION, points=points)
    print(f"Done. Indexed {point_id} chunks from {repo_root}")

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")