-- $1: blog_id
-- $2: url
-- $3: github_id
-- $4: github_name

INSERT INTO "repo" (blog_id, url, github_id, github_name)
VALUES ($1, $2, $3, $4)
ON CONFLICT (blog_id) DO UPDATE SET
	url = $2,
	github_id = $3,
	github_name = $4
RETURNING 
	id,
	blog_id, 
	url,
	github_id,
	github_name, 
	metadata,
	to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
