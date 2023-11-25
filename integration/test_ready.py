from config import url

import httpx

path = "/ready"
route = url + path


def test_ready_route_returns_ready():
    r = httpx.get(route)
    assert r.status_code == 200
