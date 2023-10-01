from config import host

import httpx

path = "/blogs"
route = host + path

def test_get_blog_post_by_blog_and_post_slugs():
    r = httpx.get(route+"/iot-insights/posts/iot-and-insights")
    assert r.status_code == 200

