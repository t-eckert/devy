import httpx
import os

client = httpx.Client(
    base_url=os.environ.get("URL") or "http://localhost:3000", follow_redirects=True
)
