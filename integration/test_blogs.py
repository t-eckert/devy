import httpx

host = "http://localhost:8000"
path = "/blogs"
route = host + path

def test_get_blog_post_by_blog_and_post_slugs():
    r = httpx.get(route+"/iot-insights/posts/iot-and-insights")
    assert r.status_code == 200

