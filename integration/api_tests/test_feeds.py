from framework.config import api
from framework.utils import is_subset

import httpx
import pytest

path = "/feeds"
route = api + path



def test_get_new_feed_posts():
    r = httpx.get(route + "/recent")

    assert r.status_code == 200


def test_get_nonexistent_feed():
    r = httpx.get(route + "/asdf")

    assert r.status_code == 404
