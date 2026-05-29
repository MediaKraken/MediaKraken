#!/usr/bin/env python3
"""Index a code repo into Qdrant for semantic search."""
import os, sys, hashlib, uuid, subprocess
from pathlib import Path
import httpx
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct, PointIdsList
import time

# python3 -m venv venv
# source ai_local/aider/venv/bin/activate 
# pip3 install httpx qdrant-client

# test embedder connection:
# curl -v --max-time 5 http://skynetgpu1:8001/v1/embeddings \
#   -H 'Content-Type: application/json' \
#   -d '{"model":"BAAI/bge-m3","input":["hello"]}'


QDRANT_URL = "http://skynetmem1:6333"
EMBED_URL = "http://skynetgpu1:8001/v1/embeddings"
EMBED_MODEL = "BAAI/bge-m3"
VECTOR_SIZE = 1024
CHUNK_LINES = 60
CHUNK_OVERLAP = 10

CODE_EXTS = {".py",".js",".ts",".tsx",".jsx",".go",".rs",".java",".c",".cc",
             ".cpp",".h",".hpp",".rb",".php",".cs",".kt",".swift",".scala",
             ".sh",".yaml",".yml",".toml",".sql",".md"}

MAX_CHARS = 4000

def gitignored(repo_root):
    try:
        out = subprocess.check_output(
            ["git","-C",repo_root,"ls-files","-o","-i","--exclude-standard"],
            text=True
        )
        return set(out.splitlines())
    except subprocess.CalledProcessError:
        return set()

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

def chunk_id(path: str, text: str) -> str:
    """Deterministic UUID derived from path + chunk content."""
    h = hashlib.sha256(f"{path}\0{text}".encode("utf-8", "ignore")).digest()
    return str(uuid.UUID(bytes=h[:16]))

def embed(texts, retries=5):
    for attempt in range(retries):
        try:
            r = httpx.post(EMBED_URL, json={"model": EMBED_MODEL, "input": texts}, timeout=600)
            r.raise_for_status()
            return [d["embedding"] for d in r.json()["data"]]
        except (httpx.ReadTimeout, httpx.ConnectError, httpx.HTTPStatusError, httpx.RemoteProtocolError) as e:
            if attempt == retries - 1:
                raise
            wait = min(30, 2 ** attempt)   # 1, 2, 4, 8, 16s
            print(f"  embed failed ({type(e).__name__}: {e}), retrying in {wait}s...")
            time.sleep(wait)

def ensure_collection(qc, name):
    if not qc.collection_exists(name):
        qc.create_collection(
            collection_name=name,
            vectors_config=VectorParams(size=VECTOR_SIZE, distance=Distance.COSINE),
        )

def fetch_all_ids(qc, collection):
    """Scroll the entire collection and return the set of existing point IDs."""
    ids = set()
    offset = None
    while True:
        records, offset = qc.scroll(
            collection_name=collection,
            limit=10000,
            offset=offset,
            with_payload=False,
            with_vectors=False,
        )
        ids.update(r.id for r in records)
        if offset is None:
            break
    return ids

def index_repo(repo_root, collection_name):
    repo_root = Path(repo_root).resolve()
    qc = QdrantClient(url=QDRANT_URL)

    ensure_collection(qc, collection_name)
    existing_ids = fetch_all_ids(qc, collection_name)
    print(f"  {collection_name}: {len(existing_ids)} existing chunks")

    ignored = gitignored(str(repo_root))

    seen_ids = set()
    batch_texts, batch_meta, batch_ids = [], [], []
    points = []
    new_count = 0
    reused_count = 0

    def flush_upsert():
        nonlocal points
        if points:
            qc.upsert(collection_name, points=points)
            points = []

    for root, dirs, files in os.walk(repo_root):
        dirs[:] = [d for d in dirs if d not in (".git","node_modules","__pycache__","venv",".venv","dist","build","target")]
        for fname in files:
            p = Path(root)/fname
            rel = str(p.relative_to(repo_root))
            if rel in ignored: continue
            if p.suffix.lower() not in CODE_EXTS: continue
            if p.stat().st_size > 500_000: continue
            for start, end, text in chunk_file(p):
                cid = chunk_id(rel, text)
                if cid in seen_ids:
                    # exact duplicate chunk within this run (rare); skip
                    continue
                seen_ids.add(cid)
                if cid in existing_ids:
                    reused_count += 1
                    continue
                batch_texts.append(text)
                batch_meta.append({"path": rel, "start": start, "end": end})
                batch_ids.append(cid)
                if len(batch_texts) >= 2:
                    vecs = embed(batch_texts)
                    for v, m, i in zip(vecs, batch_meta, batch_ids):
                        points.append(PointStruct(id=i, vector=v, payload=m))
                        new_count += 1
                    batch_texts, batch_meta, batch_ids = [], [], []
                    if len(points) >= 64:
                        flush_upsert()
                        print(f"  embedded {new_count} new (reused {reused_count})...")

    if batch_texts:
        vecs = embed(batch_texts)
        for v, m, i in zip(vecs, batch_meta, batch_ids):
            points.append(PointStruct(id=i, vector=v, payload=m))
            new_count += 1
    flush_upsert()

    stale = existing_ids - seen_ids
    if stale:
        qc.delete(
            collection_name=collection_name,
            points_selector=PointIdsList(points=list(stale)),
        )
    print(f"Done. {collection_name}: {new_count} new, {reused_count} reused, {len(stale)} removed")

def main():
    repos = [
        {"path": "/home/metaman/repositories/MediaKraken", "collection": "mediakraken"},
        {"path": "/home/metaman/repositories/bmo_core", "collection": "bmo_core"},
    ]
    for repo in repos:
        print(f"Indexing {repo['path']} -> {repo['collection']}")
        index_repo(repo["path"], repo["collection"])

if __name__ == "__main__":
    main()