# [Devy](https://devy.page)

> Blog in Markdown from your GitHub repo.

## Local Development

Devy provides a Makefile to ease development.
To run the site locally with demo data, you will need to have the following installed:

- Docker
- Rust
- Node

Start the dev database.

```sh
make db-serve
```

Start the API.

```sh
make api-serve
```

Start the site.

```sh
make site-serve
```
