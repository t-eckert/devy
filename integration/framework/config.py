import os

api_host = os.environ.get("API_HOST") or "localhost:8000"
api = f"http://{api_host}"

ui_host = os.environ.get("UI_HOST") or "localhost:3000"
ui = f"http://{ui_host}"

url = os.environ.get("URL") or "http://localhost:3000"
