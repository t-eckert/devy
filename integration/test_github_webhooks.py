import config
import httpx

path = "/api/webhooks/github/push"
route = config.HOST + path

def test_post_push_webhook():
    body = {
      "ref": "refs/heads/main",
      "before": "5aef35982fb2d34e9d9d4502f6ede1072793222d",
      "after": "a4d7d720de70822b6c49b82bbf1e1a2a1ef86fbb",
      "created": False,
      "deleted": False,
      "forced": False,
      "base_ref": None,
      "compare": "https://github.com/user/repo/compare/5aef35982fb2...a4d7d720de70",
      "commits": [
        {
          "id": "a4d7d720de70822b6c49b82bbf1e1a2a1ef86fbb",
          "tree_id": "f9d2a07e9488b91af2641b26b9407fe22a451433",
          "distinct": True ,
          "message": "Update README.md",
          "timestamp": "2023-07-16T12:34:56Z",
          "url": "https://github.com/user/repo/commit/a4d7d720de70",
          "author": {
            "name": "John Doe",
            "email": "johndoe@example.com",
            "username": "johndoe"
          },
          "committer": {
            "name": "John Doe",
            "email": "johndoe@example.com",
            "username": "johndoe"
          },
          "added": [],
          "removed": [],
          "modified": ["README.md"]
        }
      ],
      "head_commit": {
        "id": "a4d7d720de70822b6c49b82bbf1e1a2a1ef86fbb",
        "tree_id": "f9d2a07e9488b91af2641b26b9407fe22a451433",
        "distinct": True ,
        "message": "Update README.md",
        "timestamp": "2023-07-16T12:34:56Z",
        "url": "https://github.com/user/repo/commit/a4d7d720de70",
        "author": {
          "name": "John Doe",
          "email": "johndoe@example.com",
          "username": "johndoe"
        },
        "committer": {
          "name": "John Doe",
          "email": "johndoe@example.com",
          "username": "johndoe"
        },
        "added": [],
        "removed": [],
        "modified": ["README.md"]
      },
      "repository": {
        "id": 123456789,
        "node_id": "MDEwOlJlcG9zaXRvcnkxMjM0NTY3ODk=",
        "name": "repo",
        "full_name": "user/repo",
        "private": False,
        "html_url": "https://github.com/user/repo",
        "description": "Example repository",
        "url": "https://github.com/user/repo",
        "default_branch": "main"
      },
      "pusher": {
        "name": "John Doe",
        "email": "johndoe@example.com"
      },
      "sender": {
        "login": "johndoe",
        "id": 987654321,
        "node_id": "MDQ6VXNlcjMwMDIxMzk3",
        "avatar_url": "https://avatars.githubusercontent.com/u/12345678?v=4",
        "gravatar_id": "",
        "url": "https://api.github.com/users/johndoe",
        "html_url": "https://github.com/johndoe",
        "type": "User",
        "site_admin": False
      }
    }


    resp = httpx.post(route, json=body)


