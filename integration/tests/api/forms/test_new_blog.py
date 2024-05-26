from framework.client import client, with_session
from framework.payloads import payload
from framework.utils import tee
from framework.session import user_session


def test_post_new_blog():
    resp = with_session(client, user_session()).post(
        "/api/forms/new-blog", json=tee(payload("forms-new-blog"))
    )

    assert resp.status_code == 200
