from config import url

import httpx
import json

upload = {
    "repo": "github.com/t-eckert/devy.git"
}

r = httpx.post(f"{url}uploads", json=upload)

print(json.dumps(r.json()))


