from config import host

import httpx

path = "/users"
route = host + path

def test_get_user_by_username():
    r = httpx.get(route + "/t-eckert")

    assert r.status_code == 200


