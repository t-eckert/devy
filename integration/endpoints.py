import httpx
import config

def get(path:str) -> httpx.Response:
    return httpx.get(config.HOST + path)

def get_ready() -> httpx.Response:
    return httpx.get(config.HOST + '/ready')

def get_feed_by_id(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/feeds/' + id)

def get_post_by_id(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/posts/' + id)

def get_profile_by_id(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/profiles/' + id)
