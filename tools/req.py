from rich import print

import argparse
import httpx
import os

LOCAL = "http://localhost:3000"


parser = argparse.ArgumentParser(description="Request a route.")
parser.add_argument("method", type=str, help="HTTP method.")
parser.add_argument("path", type=str, help="Route path.")
parser.add_argument("--payload", type=str, help="Payload to send.")
parser.add_argument("--session", type=str, help="Session to use.")


def main():
    client = httpx.Client(
        base_url=os.environ.get("URL") or LOCAL, follow_redirects=True
    )
    args = parser.parse_args()
    response = client.request(args.method, args.path)

    print(response.status_code)
    if response.headers["Content-Type"] == "application/json":
        print(response.json())
    elif response.headers["Content-Type"] == "text/html":
        print(response.text)


if __name__ == "__main__":
    main()
