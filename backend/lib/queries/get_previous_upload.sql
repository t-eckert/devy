SELECT
    id,
	previous_upload_id,
	status,
	repo,
	sha,
	logs,
	diff,
	changeset,
	created_at,
	updated_at
FROM "upload"
WHERE
    repo=$1
    AND status='done'
ORDER BY created_at DESC
LIMIT 1;
