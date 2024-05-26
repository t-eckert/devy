from framework.client import client


def test_get_recent():
    r = client.get("/api/feeds/recent")

    assert r.status_code == 200

    data = r.json()

    assert "entries" in data
    assert "feedConfig" in data


def test_get_popular():
    r = client.get("/api/feeds/popular")

    assert r.status_code == 200

    data = r.json()

    assert "entries" in data
    assert "feedConfig" in data


def test_get_nonexistent():
    r = client.get("/api/feeds/asdf")

    assert r.status_code == 404
