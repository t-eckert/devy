import httpx
import config

def get(path:str) -> httpx.Response:
    return httpx.get(config.HOST + path)

def get_ready() -> httpx.Response:
    return httpx.get(config.HOST + '/ready')

def get_feed(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/feeds/' + id)

def get_post(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/posts/' + id)

def get_profiles(id:str) -> httpx.Response:
    return httpx.get(config.HOST + '/profiles/' + id)
