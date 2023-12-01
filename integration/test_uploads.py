from config import url
from framework import is_subset
from rich import print

import httpx
import pytest


@pytest.mark.parametrize(
    "upload, headers, expect_code, expect_json",
    [
        (
            {"repo": "https://github.com/t-eckert/devy-test-repo"},
            {"Content-Type": "application/json"},
            401,
            None,
        ),
        (
            {"repo": "https://github.com/t-eckert/devy-test-repo"},
            {"Content-Type": "application/json", "Authorization": "Bearer 123"},
            200,
            {
                "status": "done",
                "logs": [
                    "INFO: Upload received by uploader.",
                    "INFO: Repository cloned.",
                    "INFO: Cleaning up repository.",
                    "INFO: Upload complete.",
                ],
                "repo": "https://github.com/t-eckert/devy-test-repo",
            },
        ),
    ],
)
def test_post(upload, headers, expect_code, expect_json):
    r = httpx.post(f"{url}/uploads", json=upload, headers=headers)

    assert r.status_code == expect_code
    if expect_json:
        print(expect_json)
        print(r.json())
        assert is_subset(expect_json, r.json())

def test_get_by_username():
    r = httpx.get(url + "/uploads/t-eckert")

    assert r.status_code == 200
    print(r.json())
