from config import url

import httpx
import json

upload = {
    "repo": "https://github.com/t-eckert/devy-test-repo"
}

r = httpx.post(f"{url}uploads", json=upload)

if r.status_code == 200:
    print(json.dumps(r.json()))
else:
    print(f"Error: {r.status_code}")
    print(r.text)


