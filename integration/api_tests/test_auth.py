from framework.config import api
import re
import httpx
import pytest


@pytest.mark.skip()
def test_login_is_redirected():
    r = httpx.get(api + "/auth/login")

    assert r.status_code == 308

    location = r.headers["Location"]
    assert re.match(
        r"https://github.com/login/oauth/authorize\?response_type\=code&client_id\=.+&scope=repo\+user&redirect_uri\=.*",
        location,
    )
