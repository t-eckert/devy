from config import url

import httpx

upload = {
    "repo": "github.com/t-eckert/devy.git"
}

r = httpx.post(f"{url}uploads", json=upload)

print(r.status_code)
print(r.json())


