from framework.client import client
import json

github_push_webhook = None
with open("./testdata/github_push_webhook.json") as f:
    github_push_webhook = json.load(f)


def test_webhooks_with_github_post_webhook():
    print(github_push_webhook)
