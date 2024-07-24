# [Devy](https://devy.page)

> Blog in Markdown from your GitHub repo.

Devy is a platform for developers to share their knowledge and ideas.

## Local Development

Devy provides a Makefile to ease development.
To run the site locally with demo data, you will need to have the following installed:

- Docker
- Rust
- Node

Start the devlopment database. This database is seeded with users, posts,
and other demo data.

```sh
make db-serve
```

Start the API. This mediates all connections between the frontend and the
database, provides authentication, and processes GitHub webhooks for post
uploads.

```sh
make api-serve
```

Start the frontend. This is the user interface for the site and proxies API
requests to the API.

```sh
make frontend-serve
```
