from config import url

import httpx
import pytest

path = "/profiles"
route = url + path


def test_get_profile_by_username():
    r = httpx.get(route + "/t-eckert")

    assert r.status_code == 200


@pytest.mark.skip(reason="Need to update expected")
def test_get_blogs_by_username():
    expected = [
        {
            "name": "Machine Learning",
            "slug": "machine-learning",
            "username": "t-eckert",
            "displayName": "Thomas Eckert",
            "description": None,
            "createdAt": "2023-10-07T02:10:19.19",
            "updatedAt": "2023-10-07T02:10:19.19",
        },
        {
            "name": "DevOops",
            "slug": "devoops",
            "username": "t-eckert",
            "displayName": "Thomas Eckert",
            "description": None,
            "createdAt": "2023-10-07T02:10:19.19",
            "updatedAt": "2023-10-07T02:10:19.19",
        },
    ]

    r = httpx.get(route + "/t-eckert/blogs")

    assert r.status_code == 200
    assert r.json() == expected


def test_get_all_liked_post_ids_for_profile():
    r = httpx.get(route + "/e2f0fa7e-4517-4ac8-bbc6-73067d3feed4/likes/ids")
    print(r.json())

    assert r.status_code == 200
