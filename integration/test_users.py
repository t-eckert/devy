from config import url

import httpx

path = "/users"
route = url + path


def test_get_user_by_username():
    r = httpx.get(route + "/t-eckert")

    assert r.status_code == 200
