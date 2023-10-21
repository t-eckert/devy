from config import host

import httpx


def test_post_is_served_with_likes():
    r = httpx.get(host + "/blogs/iot-insights/posts/iot-and-insights")

    assert r.status_code == 200
    print(r.json())


