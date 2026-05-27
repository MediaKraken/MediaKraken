#!/usr/bin/env python3
"""MCP server: semantic code search via llama.cpp embedder + Qdrant."""
import os
import httpx
from mcp.server.fastmcp import FastMCP
from qdrant_client import QdrantClient

QDRANT_URL    = os.getenv("QDRANT_URL", "http://qdrant:6333")
EMBED_URL     = os.getenv("EMBED_URL",  "http://embedder:8001/v1/embeddings")
EMBED_MODEL   = os.getenv("EMBED_MODEL", "qwen3-embedding")
DEFAULT_COLL  = os.getenv("DEFAULT_COLLECTION", "code")
LIMIT         = int(os.getenv("SEARCH_LIMIT", "8"))

qdrant = QdrantClient(url=QDRANT_URL)
mcp = FastMCP("qdrant-code-search", host="0.0.0.0", port=8000)


def embed(text: str) -> list[float]:
    r = httpx.post(
        EMBED_URL,
        json={"model": EMBED_MODEL, "input": [text]},
        timeout=60,
    )
    r.raise_for_status()
    return r.json()["data"][0]["embedding"]


@mcp.tool()
def qdrant_find(query: str, collection: str | None = None) -> str:
    """Search the indexed code repository for snippets relevant to the query.

    Use this BEFORE writing or modifying code so new code matches existing
    patterns in the codebase. Returns up to 8 ranked snippets with file paths
    and line ranges.

    Args:
        query: Natural-language description of what you're looking for.
               Example: "jwt token refresh middleware"
        collection: Optional Qdrant collection name. Omit to use the repo's
                    default collection configured for this MCP server.
    """
    coll = collection or DEFAULT_COLL
    try:
        vec = embed(query)
    except Exception as e:
        return f"Embedding failed: {e}"

    try:
        hits = qdrant.search(
            collection_name=coll, query_vector=vec, limit=LIMIT
        )
    except Exception as e:
        return f"Qdrant search failed on collection '{coll}': {e}"

    if not hits:
        return f"No matches in collection '{coll}'."

    out = [f"Found {len(hits)} matches in '{coll}':\n"]
    for i, h in enumerate(hits, 1):
        p = h.payload or {}
        path  = p.get("path", "?")
        start = p.get("start_line", "?")
        end   = p.get("end_line", "?")
        body  = (p.get("text") or p.get("content") or "")[:1500]
        out.append(
            f"--- [{i}] {path}:{start}-{end}  (score={h.score:.3f}) ---\n{body}"
        )
    return "\n\n".join(out)


@mcp.tool()
def qdrant_list_collections() -> str:
    """List all Qdrant collections available for search."""
    cols = qdrant.get_collections().collections
    if not cols:
        return "No collections found."
    return "\n".join(f"- {c.name}" for c in cols)


if __name__ == "__main__":
    # streamable-http is the modern remote transport; Codex supports it.
    mcp.run(transport="streamable-http")