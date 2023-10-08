from config import host

import httpx

path = "/ready"
route = host + path


def test_ready_route_returns_ready():
    r = httpx.get(route)
    assert r.status_code == 200
