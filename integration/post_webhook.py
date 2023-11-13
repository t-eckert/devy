"""Based on https://docs.github.com/en/webhooks/webhook-events-and-payloads"""

from config import url

import httpx
import json

headers = {
    "X-GitHub-Event": "push"
}
webhook = {
  "ref": "refs/heads/main",
  "before": "a453e51a9c7f9a9a5b8d33fbf5d61a1d4e2d6c01",
  "after": "6dcb09b5b57875f334f61aebed695e2e4193db5e",
  "created": False,
  "deleted": False,
  "forced": False,
  "base_ref": None,
  "compare": "https://github.com/example/repo/compare/a453e51a9c7f...6dcb09b5b578",
  "commits": [
    {
      "id": "6dcb09b5b57875f334f61aebed695e2e4193db5e",
      "tree_id": "f9d2a07e9488b91af2641b26b9bc8a58b79f87f9",
      "distinct": True,
      "message": "Update README.md",
      "timestamp": "2023-11-12T15:38:54-08:00",
      "url": "https://github.com/example/repo/commit/6dcb09b5b578",
      "author": {
        "name": "John Doe",
        "email": "john.doe@example.com",
        "username": "johndoe"
      },
      "committer": {
        "name": "John Doe",
        "email": "john.doe@example.com",
        "username": "johndoe"
      },
      "added": [],
      "removed": [],
      "modified": ["README.md"]
    }
  ],
  "head_commit": {
    "id": "6dcb09b5b57875f334f61aebed695e2e4193db5e",
    "tree_id": "f9d2a07e9488b91af2641b26b9bc8a58b79f87f9",
    "distinct": True,
    "message": "Update README.md",
    "timestamp": "2023-11-12T15:38:54-08:00",
    "url": "https://github.com/example/repo/commit/6dcb09b5b578",
    "author": {
      "name": "John Doe",
      "email": "john.doe@example.com",
      "username": "johndoe"
    },
    "committer": {
      "name": "John Doe",
      "email": "john.doe@example.com",
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
    "full_name": "example/repo",
    "private": False,
    "owner": {
      "name": "example",
      "email": "admin@example.com"
    },
    "html_url": "https://github.com/example/repo",
    "description": "Example Repository",
    "fork": False,
    "url": "https://github.com/example/repo",
    "forks_url": "https://api.github.com/repos/example/repo/forks",
    "keys_url": "https://api.github.com/repos/example/repo/keys{/key_id}",
    "collaborators_url": "https://api.github.com/repos/example/repo/collaborators{/collaborator}",
    "teams_url": "https://api.github.com/repos/example/repo/teams",
    "hooks_url": "https://api.github.com/repos/example/repo/hooks",
    "issue_events_url": "https://api.github.com/repos/example/repo/issues/events{/number}",
    "events_url": "https://api.github.com/repos/example/repo/events",
    "assignees_url": "https://api.github.com/repos/example/repo/assignees{/user}",
    "branches_url": "https://api.github.com/repos/example/repo/branches{/branch}",
    "tags_url": "https://api.github.com/repos/example/repo/tags",
    "blobs_url": "https://api.github.com/repos/example/repo/git/blobs{/sha}",
    "git_tags_url": "https://api.github.com/repos/example/repo/git/tags{/sha}",
    "git_refs_url": "https://api.github.com/repos/example/repo/git/refs{/sha}",
    "trees_url": "https://api.github.com/repos/example/repo/git/trees{/sha}",
    "statuses_url": "https://api.github.com/repos/example/repo/statuses/{sha}",
    "languages_url": "https://api.github.com/repos/example/repo/languages",
    "stargazers_url": "https://api.github.com/repos/example/repo/stargazers",
    "contributors_url": "https://api.github.com/repos/example/repo/contributors",
    "subscribers_url": "https://api.github.com/repos/example/repo/subscribers",
    "subscription_url": "https://api.github.com/repos/example/repo/subscription",
    "commits_url": "https://api.github.com/repos/example/repo/commits{/sha}",
    "git_commits_url": "https://api.github.com/repos/example/repo/git/commits{/sha}",
    "comments_url": "https://api.github.com/repos/example/repo/comments{/number}",
    "issue_comment_url": "https://api.github.com/repos/example/repo/issues/comments{/number}",
    "contents_url": "https://api.github.com/repos/example/repo/contents/{+path}",
    "compare_url": "https://api.github.com/repos/example/repo/compare/{base}...{head}",
    "merges_url": "https://api.github.com/repos/example/repo/merges",
    "archive_url": "https://api.github.com/repos/example/repo/{archive_format}{/ref}",
    "downloads_url": "https://api.github.com/repos/example/repo/downloads",
    "issues_url": "https://api.github.com/repos/example/repo/issues{/number}",
    "pulls_url": "https://api.github.com/repos/example/repo/pulls{/number}",
    "milestones_url": "https://api.github.com/repos/example/repo/milestones{/number}",
    "notifications_url": "https://api.github.com/repos/example/repo/notifications{?since,all,participating}",
    "labels_url": "https://api.github.com/repos/example/repo/labels{/name}",
    "releases_url": "https://api.github.com/repos/example/repo/releases{/id}",
    "deployments_url": "https://api.github.com/repos/example/repo/deployments",
    "created_at": 1577853690,
    "updated_at": "2023-11-12T15:38:54-08:00",
    "pushed_at": 1665692334,
    "git_url": "git://github.com/example/repo.git",
    "ssh_url": "git@github.com:example/repo.git",
    "clone_url": "https://github.com/example/repo.git",
    "svn_url": "https://github.com/example/repo",
    "homepage": None,
    "size": 108,
    "stargazers_count": 0,
    "watchers_count": 0,
    "language": "JavaScript",
    "has_issues": True,
    "has_projects": True,
    "has_downloads": True,
    "has_wiki": True,
    "has_pages": False,
    "forks_count": 0,
    "mirror_url": None,
    "archived": False,
    "disabled": False,
    "open_issues_count": 1,
    "license": None,
    "forks": 0,
    "open_issues": 1,
    "watchers": 0,
    "default_branch": "main"
  },
  "pusher": {
    "name": "johndoe",
    "email": "john.doe@example.com"
  },
  "sender": {
    "login": "johndoe",
    "id": 123456,
    "node_id": "MDQ6VXNlcjEyMzQ1Ng==",
    "avatar_url": "https://avatars.githubusercontent.com/u/123456?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/johndoe",
    "html_url": "https://github.com/johndoe",
    "followers_url": "https://api.github.com/users/johndoe/followers",
    "following_url": "https://api.github.com/users/johndoe/following{/other_user}",
    "gists_url": "https://api.github.com/users/johndoe/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/johndoe/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/johndoe/subscriptions",
    "organizations_url": "https://api.github.com/users/johndoe/orgs",
    "repos_url": "https://api.github.com/users/johndoe/repos",
    "events_url": "https://api.github.com/users/johndoe/events{/privacy}",
    "received_events_url": "https://api.github.com/users/johndoe/received_events",
    "type": "User",
    "site_admin": False
  }
}

r = httpx.post(f"{url}webhooks", headers=headers, json=webhook)

print(r.status_code)


