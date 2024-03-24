-- $1: blog_id
-- $2: url

INSERT INTO "repo" (blog_id, url)
VALUES ($1, $2)
ON CONFLICT (blog_id) DO UPDATE SET
	url = $2
RETURNING
	id,
	blog_id,
	url,
	metadata,
	to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
