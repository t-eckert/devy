from framework.config import api
from framework.client import client


def test_get_new_feed_posts():
    r = client.get("/api/feeds/recent")

    assert r.status_code == 200


def test_get_nonexistent_feed():
    r = client.get("/api/feeds/asdf")

    assert r.status_code == 404
