from config import url

import httpx

path = "/forms"
route = url + path

def test_new_blog():
    r = httpx.post(route + "/new-blog", json={
        "username": "t-eckert",
        "name": "Test Blog",
        "repoUrl": "https://github.com/t-eckert/devy-test-repo",
    }, headers={
        "content-type": "application/x-www-form-urlencoded"
    })

    assert r.status_code == 201


