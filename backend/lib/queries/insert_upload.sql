insert into "upload" (previous_upload_id, repo)
values ($1, $2)
returning id;
