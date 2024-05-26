from framework.client import client


def test_ready_route_returns_ready():
    resp = client.get("/api/ready")
    assert resp.status_code == 200
