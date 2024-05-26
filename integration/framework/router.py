from typing import Callable
from framework.client import client


def route(method: str, path: str) -> Callable:
    return lambda: client.request(method, path)
