from config import url

import httpx
import json


r = httpx.get(f"{url}uploads/t-eckert")

print(json.dumps(r.json()))


