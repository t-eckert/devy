import httpx

host = "http://localhost:8000"
path = "/feeds"
route = host + path

def test_get_feed_new():
    r = httpx.get(route+"/new")
    assert r.status_code == 200
    assert r.json() == {"id":"new", "name":"New"}


def test_get_feed_popular():
    r = httpx.get(route+"/popular")
    assert r.status_code == 200
    assert r.json() == {"id":"popular", "name":"Popular"}

def test_get_feed_posts_limit():
    r = httpx.get(route+"/new?limit=10")
    assert r.status_code == 200
    assert len(r.json()["posts"]) == 10
