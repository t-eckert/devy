select
	id,
	previous_upload_id,
	status,
	repo,
	sha,
	logs,
	created_at,
	updated_at
from "upload"
where repo = $1;
