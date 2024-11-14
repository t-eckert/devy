insert into "upload" (previous_upload_id, blog_id, repo)
values ($1, $2, $3)
returning id;
