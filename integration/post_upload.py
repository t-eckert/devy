from config import url

import httpx
import json

upload = {
    "repo": "https://github.com/t-eckert/devy"
}

r = httpx.post(f"{url}uploads", json=upload)

print(json.dumps(r.json()))


