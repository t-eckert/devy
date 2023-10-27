from config import url

import httpx

def tests_post_can_be_liked():
    like = {
        "postId": "2d0be777-545f-4dbe-95c9-768670e6caba",
        "profileId": "e2f0fa7e-4517-4ac8-bbc6-73067d3feed4"
    }

    r = httpx.post(url + "likes", json=like)

    assert r.status_code == 200
    assert r.json() == like


def tests_post_can_be_unliked():
    like = {
        "postId": "2d0be777-545f-4dbe-95c9-768670e6caba",
        "profileId": "e2f0fa7e-4517-4ac8-bbc6-73067d3feed4"
    }

    r = httpx.delete(url + f"likes/{like['postId']}/{like['profileId']}")

    assert r.status_code == 200
    assert r.json() == like

