from framework.config import api
from framework.utils import is_subset

import httpx
import pytest

path = "/feeds"
route = api + path


def test_get_new_feed_config():
    r = httpx.get(route + "/new/config")

    assert r.status_code == 200
    assert is_subset(
        r.json(),
        {
            "id": "5941b29d-246d-4897-a69e-3201f6ad8715",
            "name": "New",
        },
    )


def test_get_popular_feed_config():
    r = httpx.get(route + "/popular/config")

    assert r.status_code == 200
    assert is_subset(
        r.json(),
        {
            "id": "e9173695-1b31-465f-9e79-a80224be44ad",
            "name": "Popular",
        },
    )


def test_get_new_feed_posts():
    r = httpx.get(route + "/new/posts")

    assert r.status_code == 200
    assert len(r.json()) == 30


def test_get_feed_posts_limit():
    r = httpx.get(route + "/new/posts?limit=10")

    assert r.status_code == 200
    assert len(r.json()) == 10


@pytest.mark.skip("Limit not implemented")
def test_get_feed_posts_limit_and_offset():
    r = httpx.get(route + "/new/posts?limit=11?offset=10")

    assert r.status_code == 200
    assert len(r.json()) == 11


def test_get_nonexistent_feed():
    r = httpx.get(route + "/asdf")

    assert r.status_code == 404
