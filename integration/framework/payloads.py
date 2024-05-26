import os

current_dir = os.path.dirname(__file__)


def payload(name: str) -> str:
    with open(f"{current_dir}/payloads/{name}.json") as f:
        return f.read()
