insert into "upload" (previous_upload_id, blog_id)
values ($1, $2)
returning id;
