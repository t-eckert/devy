select
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
from "upload"
where repo = $1;
