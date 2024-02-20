from framework.config import api
import httpx
import pytest


@pytest.mark.skip(reason="not implemented")
def test_get_user_by_username():
    r = httpx.get(api + "/users/t-eckert")

    assert r.status_code == 200
