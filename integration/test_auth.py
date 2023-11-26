from config import url
from rich import print
import re
from framework import is_subset

import httpx

def test_login_is_redirected():
    r = httpx.get(url + "/auth/login")

    assert r.status_code == 308

    location = r.headers["Location"]
    assert re.match(r"https://github.com/login/oauth/authorize\?response_type\=code&client_id\=.+&scope=repo\+user&redirect_uri\=.*", location)


