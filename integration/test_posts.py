from config import url

import httpx


def test_post_is_served_with_likes():
    r = httpx.get(url + "blogs/iot-insights/posts/iot-and-insights")

    assert r.status_code == 200
    print(r.json())


