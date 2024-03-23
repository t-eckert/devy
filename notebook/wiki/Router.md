# API

Should I add a `meta` value to responses? Maybe in the future.
It would help a lot with pagination.

```json
{
	"meta": {
		"count": 30,
		"previousPage": "https://api.devy.page/v1/feeds/new/posts?limit=30&offset=60",
		"nextPage": "https://api.devy.page/v1/feeds/new/posts?limit=30&offset=120"
	}	
}
```

## `GET /auth/login` 

Forwards the request to GitHub Oauth endpoint.

## `GET /auth/callback`


## `GET /blogs/:blog_slug`


### `GET /blogs/:blog_slug/posts`

Returns all posts for a blog by its slug.
### `GET /blogs/:blog_slug/posts/:post_slug`

Returns a post by its blog and post slug.

### `GET /feeds/new/posts`

### `GET /feeds/popular/posts`

### `GET /feeds/new/config`

### `GET feeds/popular/config`

### `GET /profiles/:username`

### `GET /profiles/:username/blogs`

### `GET /profiles/:username/posts`

### `GET /profiles/:username/likes`

### `GET /profiles/:username/likes/ids`

### `GET /users/:username`

### `POST /webhooks`

### `DELETE /blogs/:blog_slug`

### `POST /forms/new-blog`

### `POST /likes

### `DELETE /likes/:post_id/:profile_id`

### `POST /repos`



