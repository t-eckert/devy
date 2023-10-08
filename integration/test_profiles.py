from config import host

import httpx

path = "/profiles"
route = host + path


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
