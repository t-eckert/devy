from framework.session import Session

import httpx
import os

local = "http://localhost:3000"

client = httpx.Client(base_url=os.environ.get("URL") or local, follow_redirects=True)


def with_session(client: httpx.Client, session: Session) -> httpx.Client:
    client.headers["Authorization"] = f"Bearer {session.jwt()}"
    return client
