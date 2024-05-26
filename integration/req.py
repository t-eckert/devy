from rich import print
from framework.router import route

import argparse

parser = argparse.ArgumentParser(description="Request a route.")
parser.add_argument("method", type=str, help="HTTP method.")
parser.add_argument("path", type=str, help="Route path.")
parser.add_argument("--payload", type=str, help="Payload to send.")
parser.add_argument("--session", type=str, help="Session to use.")

if __name__ == "__main__":
    args = parser.parse_args()
    request = route(args.method, args.path)
    response = request()
    print(response.status_code)
    if response.headers["Content-Type"] == "application/json":
        print(response.json())
    elif response.headers["Content-Type"] == "text/html":
        print(response.text)
