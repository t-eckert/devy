from framework.client import client


def test_homepage_loads():
    resp = client.get("/")
    assert resp.status_code == 200
