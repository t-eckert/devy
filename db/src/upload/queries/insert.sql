INSERT INTO "upload" (previous_upload_id, repo)
VALUES ($1, $2)
RETURNING
	id,
	previous_upload_id,
	status,
	repo,
	sha,
	logs,
	to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
