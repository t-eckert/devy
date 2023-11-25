from config import url

import httpx


def test_get_uploads_by_username():
    r = httpx.get(url + "/uploads/t-eckert")

    assert r.status_code == 200
    print(r.json())


