import os

host = os.environ.get('HOST') or "localhost:8000"
version = "v1"

url = f"http://{host}/{version}/"
