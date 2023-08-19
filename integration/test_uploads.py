import config
import httpx

path = "/api/uploads"
route = config.API + path


def test_create_upload():
    resp = httpx.post(route, json={"user": "Thomas Eckert"})


def test_read_upload():
    resp = httpx.get(route + "/8241b7ff-7ae5-4c5c-8695-e5de6a6649ff")
