select
	id,
	previous_upload_id,
	uploader,
	blog_id,
	status,
	sha,
	logs,
	diff,
	changeset,
	created_at,
	updated_at
from "upload"
where id = $1;
