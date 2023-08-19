import config
import httpx

path = "/likes"
route = config.API + path

def test_seeded_likes():
    ...

def test_post_like():
    """When a user likes a post, the endpoint will return the same like."""

    like = {"profileId": 1, "postId": 1}

    resp = httpx.post(route, json=like)
    assert resp.status_code == 200
    assert resp.json() == like


def test_post_like_is_idempotent():
    """When a user likes a post that has already been liked, the endpoint will accept the request idempotently."""

    like = {"profileId": 2, "postId": 2}

    resp = httpx.post(route, json=like)
    assert resp.status_code == 200
    assert resp.json() == like

    resp = httpx.post(route, json=like)
    assert resp.status_code == 200
    assert resp.json() == like


def test_delete_like():
    """When a user unlikes a post, the endpoint will return the same like."""

    like = {"profileId": 3, "postId": 3}

    resp = httpx.post(route, json=like)
    assert resp.status_code == 200
    assert resp.json() == like

    resp = httpx.delete(f"{route}/{like['profileId']}/{like['postId']}")
    assert resp.status_code == 200
    assert resp.json() == like
