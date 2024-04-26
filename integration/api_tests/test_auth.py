from framework.client import client
import re
import pytest


@pytest.mark.skip()
def test_login_is_redirected():
    r = client.get("/api/auth/login")

    assert r.status_code == 303

    location = r.headers["Location"]
    assert re.match(
        r"https://github.com/login/oauth/authorize\?response_type\=code&client_id\=.+&scope=repo\+user&redirect_uri\=.*",
        location,
    )
