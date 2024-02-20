from framework.config import api
import httpx


def test_ready_route_returns_ready():
    r = httpx.get(api + "/ready")
    assert r.status_code == 200
