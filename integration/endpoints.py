import httpx
import config


def get(path: str) -> httpx.Response:
    return httpx.get(config.API + path)


def get_ready() -> httpx.Response:
    return httpx.get(config.API + "/ready")


def get_feed_by_id(id: str) -> httpx.Response:
    return httpx.get(config.API + "/feeds/" + id)


def get_post_by_id(id: str) -> httpx.Response:
    return httpx.get(config.API + "/posts/" + id)


def get_profile_by_id(id: str) -> httpx.Response:
    return httpx.get(config.API + "/profiles/" + id)


def post_like() -> httpx.Response:
    return httpx.post(
        config.API + "/likes",
        json={
            "user_id": "365739d6-f59d-4970-b3a3-758e9df9ae07",
            "post_id": "e9ed7fb3-585f-427e-b950-d828a61cf477",
        },
    )
