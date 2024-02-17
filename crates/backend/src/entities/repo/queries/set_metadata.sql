-- $1: id
-- $2: metadata

INSERT INTO "repo" (id, metadata)
VALUES ($1, $2)
RETURNING 
	id,
	blog_id, 
	url,
	github_id,
	github_name, 
	metadata,
	to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;

