SELECT
	id,
	previous_upload_id,
	status,
	repo,
	logs,
	to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM "upload"
WHERE id=$1;

