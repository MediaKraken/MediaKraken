#!/usr/bin/env python3
"""Index a code repo into Qdrant for semantic search (async)."""
import os, sys, hashlib, uuid, subprocess, asyncio
from pathlib import Path
import httpx
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct, PointIdsList

# python3 -m venv venv
# source ai_local/aider/venv/bin/activate
# pip3 install httpx qdrant-client

QDRANT_URL = "http://skynetmem1:6333"
EMBED_URL = "http://skynetgpu1:8001/v1/embeddings"   # consider hardcoding IP to avoid DNS hiccups
EMBED_MODEL = "BAAI/bge-m3"
VECTOR_SIZE = 1024
CHUNK_LINES = 60
CHUNK_OVERLAP = 10
MAX_CHARS = 4000

EMBED_BATCH = 16                 # chunks per HTTP request
CONCURRENCY = 4                  # parallel requests; server is configured for 8
UPSERT_BATCH = 256               # points per qdrant upsert
QUEUE_MAXSIZE = CONCURRENCY * 4  # backpressure for the file walker

CODE_EXTS = {".py",".js",".ts",".tsx",".jsx",".go",".rs",".java",".c",".cc",
             ".cpp",".h",".hpp",".rb",".php",".cs",".kt",".swift",".scala",
             ".sh",".yaml",".yml",".toml",".sql",".md"}

SKIP_DIRS = {".git","node_modules","__pycache__","venv",".venv","dist","build","target"}


def gitignored(repo_root):
    try:
        out = subprocess.check_output(
            ["git","-C",repo_root,"ls-files","-o","-i","--exclude-standard"],
            text=True,
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
        block = lines[i:i + CHUNK_LINES]
        if any(l.strip() for l in block):
            text = "\n".join(block)[:MAX_CHARS]
            yield i + 1, i + len(block), text
        i += CHUNK_LINES - CHUNK_OVERLAP


def chunk_id(path: str, text: str) -> str:
    h = hashlib.sha256(f"{path}\0{text}".encode("utf-8", "ignore")).digest()
    return str(uuid.UUID(bytes=h[:16]))


async def embed_async(client: httpx.AsyncClient, texts, retries=6):
    for attempt in range(retries):
        try:
            r = await client.post(
                EMBED_URL,
                json={"model": EMBED_MODEL, "input": texts},
                timeout=600,
            )
            if r.status_code == 429:
                # Server is overloaded — back off with jitter, don't count as a hard failure
                retry_after = float(r.headers.get("retry-after", 0)) or (2 ** attempt)
                wait = min(30, retry_after) + (0.1 * attempt)  # tiny jitter
                print(f"  429 from server, backing off {wait:.1f}s...")
                await asyncio.sleep(wait)
                continue
            r.raise_for_status()
            return [d["embedding"] for d in r.json()["data"]]
        except (httpx.ReadTimeout, httpx.ConnectError,
                httpx.HTTPStatusError, httpx.RemoteProtocolError) as e:
            if attempt == retries - 1:
                raise
            wait = min(30, 2 ** attempt)
            print(f"  embed failed ({type(e).__name__}: {e}), retrying in {wait}s...")
            await asyncio.sleep(wait)


def ensure_collection(qc, name):
    if not qc.collection_exists(name):
        qc.create_collection(
            collection_name=name,
            vectors_config=VectorParams(size=VECTOR_SIZE, distance=Distance.COSINE),
        )


def fetch_all_ids(qc, collection):
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


def walk_chunks(repo_root: Path, ignored: set, existing_ids: set, stats: dict):
    """Generator yielding (cid, text, meta) tuples for chunks that need embedding."""
    seen_ids = set()
    for root, dirs, files in os.walk(repo_root):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for fname in files:
            p = Path(root) / fname
            rel = str(p.relative_to(repo_root))
            if rel in ignored:
                continue
            if p.suffix.lower() not in CODE_EXTS:
                continue
            try:
                if p.stat().st_size > 500_000:
                    continue
            except OSError:
                continue
            for start, end, text in chunk_file(p):
                cid = chunk_id(rel, text)
                if cid in seen_ids:
                    continue
                seen_ids.add(cid)
                if cid in existing_ids:
                    stats["reused"] += 1
                    continue
                yield cid, text, {"path": rel, "start": start, "end": end}
    stats["seen_ids"] = seen_ids


async def producer(repo_root, ignored, existing_ids, stats, queue):
    """Walks files, batches chunks, and pushes batches onto the queue."""
    batch_texts, batch_meta, batch_ids = [], [], []
    # walk_chunks is a sync generator; run iteration in a thread so we don't block.
    def collect():
        return list(walk_chunks(repo_root, ignored, existing_ids, stats))
    all_chunks = await asyncio.to_thread(collect)

    for cid, text, meta in all_chunks:
        batch_texts.append(text)
        batch_meta.append(meta)
        batch_ids.append(cid)
        if len(batch_texts) >= EMBED_BATCH:
            await queue.put((batch_texts, batch_meta, batch_ids))
            batch_texts, batch_meta, batch_ids = [], [], []
    if batch_texts:
        await queue.put((batch_texts, batch_meta, batch_ids))


async def worker(name, queue, client, qc, collection, stats, upsert_buf, upsert_lock):
    while True:
        item = await queue.get()
        if item is None:
            queue.task_done()
            return
        texts, metas, ids = item
        try:
            vecs = await embed_async(client, texts)
        except Exception as e:
            print(f"  [{name}] giving up on batch after retries: {e}")
            queue.task_done()
            continue

        points = [PointStruct(id=i, vector=v, payload=m)
                  for v, m, i in zip(vecs, metas, ids)]

        async with upsert_lock:
            upsert_buf.extend(points)
            stats["new"] += len(points)
            if len(upsert_buf) >= UPSERT_BATCH:
                to_send = upsert_buf[:]
                upsert_buf.clear()
                await asyncio.to_thread(qc.upsert, collection, to_send)
                print(f"  embedded {stats['new']} new (reused {stats['reused']})...")
        queue.task_done()


async def index_repo(repo_root, collection_name):
    repo_root = Path(repo_root).resolve()
    qc = QdrantClient(url=QDRANT_URL)
    ensure_collection(qc, collection_name)
    existing_ids = fetch_all_ids(qc, collection_name)
    print(f"  {collection_name}: {len(existing_ids)} existing chunks")
    ignored = gitignored(str(repo_root))

    stats = {"new": 0, "reused": 0, "seen_ids": set()}
    queue: asyncio.Queue = asyncio.Queue(maxsize=QUEUE_MAXSIZE)
    upsert_buf: list = []
    upsert_lock = asyncio.Lock()

    limits = httpx.Limits(max_connections=CONCURRENCY + 2,
                          max_keepalive_connections=CONCURRENCY + 2)
    async with httpx.AsyncClient(limits=limits) as client:
        workers = [
            asyncio.create_task(
                worker(f"w{i}", queue, client, qc, collection_name,
                       stats, upsert_buf, upsert_lock)
            )
            for i in range(CONCURRENCY)
        ]

        await producer(repo_root, ignored, existing_ids, stats, queue)
        await queue.join()

        for _ in workers:
            await queue.put(None)
        await asyncio.gather(*workers)

        # flush remaining points
        if upsert_buf:
            await asyncio.to_thread(qc.upsert, collection_name, upsert_buf)

    stale = existing_ids - stats["seen_ids"]
    if stale:
        qc.delete(
            collection_name=collection_name,
            points_selector=PointIdsList(points=list(stale)),
        )
    print(f"Done. {collection_name}: {stats['new']} new, "
          f"{stats['reused']} reused, {len(stale)} removed")


async def main():
    repos = [
        {"path": "/home/metaman/repositories/MediaKraken", "collection": "mediakraken"},
        {"path": "/home/metaman/repositories/bmo_core", "collection": "bmo_core"},
    ]
    for repo in repos:
        print(f"Indexing {repo['path']} -> {repo['collection']}")
        await index_repo(repo["path"], repo["collection"])


if __name__ == "__main__":
    asyncio.run(main())