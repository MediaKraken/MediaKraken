import httpx
r = httpx.post("http://skynetgpu1:8001/v1/embeddings",
               json={"model":"BAAI/bge-m3","input":["hello"]},
               timeout=10, trust_env=False)
print(r.status_code, len(r.json()["data"][0]["embedding"]))