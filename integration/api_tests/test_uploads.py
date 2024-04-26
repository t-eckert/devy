from framework.client import client


def test_upload_devylog():
    resp = client.post(
        "/api/uploads", json={"repo": "https://github.com/t-eckert/devylog"}
    )
    assert resp.status_code == 200
