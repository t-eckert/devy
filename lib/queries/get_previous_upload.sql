SELECT
    id,
    previous_upload_id,
    blog_id,
    uploader,
	status,
	sha,
	logs,
	diff,
	changeset,
	created_at,
	updated_at
FROM "upload"
WHERE
    blog_id=$1
    AND status='done'
ORDER BY created_at DESC
LIMIT 1;
