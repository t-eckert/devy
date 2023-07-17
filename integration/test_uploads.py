import config
import httpx

path = "/api/uploads"
route = config.HOST + path

def test_create_upload():
    resp = httpx.post(route, json={
        "user": "Thomas Eckert"
    })


