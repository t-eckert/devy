from framework.config import api
import httpx
import pytest

path = "/forms"
route = api + path


@pytest.mark.skip(reason="not implemented")
def test_new_blog():
    r = httpx.post(
        route + "/new-blog",
        data={
            "username": "t-eckert",
            "name": "Test Blog",
            "repoUrl": "https://github.com/t-eckert/devy-test-repo",
            "githubId": 1234567890,
            "githubName": "t-eckert",
        },
        headers={
            "content-type": "application/x-www-form-urlencoded",
            "authorization": "Bearer 123",
        },
    )

    assert r.status_code == 200
