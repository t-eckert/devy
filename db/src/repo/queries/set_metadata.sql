-- $1: id
-- $2: metadata

INSERT INTO "repo" (id, metadata)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE SET
    metadata = $2
RETURNING
	id,
	blog_id,
	url,
	metadata,
	to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
