select
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
from "upload"
where id = $1;
