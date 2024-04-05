SELECT
	id,
	previous_upload_id,
	status,
	repo,
	sha,
	logs,
	to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM "upload"
WHERE
    repo=$1
    AND status='done'
ORDER BY created_at DESC
LIMIT 1;
