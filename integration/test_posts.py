from config import host

import httpx


def test_post_is_served_with_likes():
    r = httpx.get(host + "/blogs/iot-insights/posts/iot-and-insights")

    assert r.status_code == 200
    print(r.json())


def tests_post_can_be_liked():
    r = httpx.post(host + "/posts/2d0be777-545f-4dbe-95c9-768670e6caba/likes", 
        headers={"profile_id": 'e2f0fa7e-4517-4ac8-bbc6-73067d3feed4'}
    )

    assert r.status_code == 200
