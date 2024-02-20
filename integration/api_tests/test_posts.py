from framework.config import api
import httpx
import pytest


@pytest.mark.skip(reason="not implemented")
def test_post_is_served_with_likes():
    r = httpx.get(api + "/blogs/iot-insights/posts/iot-and-insights")

    assert r.status_code == 200
