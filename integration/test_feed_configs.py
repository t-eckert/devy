from config import url
from framework import is_subset

import httpx


def test_get_new_feed_config():
    r = httpx.get(url + "/feeds/new/config")

    assert r.status_code == 200
    assert is_subset(
        r.json(),
        {
            "id": "5941b29d-246d-4897-a69e-3201f6ad8715",
            "name": "New",
        },
    )


def test_get_popular_feed_config():
    r = httpx.get(url + "/feeds/popular/config")

    assert r.status_code == 200
    assert is_subset(
        r.json(),
        {
            "id": "e9173695-1b31-465f-9e79-a80224be44ad",
            "name": "Popular",
        },
    )
