insert into "post" (blog_id, title, slug, body, is_draft)
values ($1, $2, $3, $4, $5)
returning id;
