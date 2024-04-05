UPDATE "upload"
SET status=$2, updated_at=NOW()
WHERE id=$1
RETURNING
	id,
	previous_upload_id,
	status,
	repo,
	sha,
	logs,
	to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
