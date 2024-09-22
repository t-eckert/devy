-- $1: blog_id
-- $2: url

insert into "repo" (blog_id, url)
values ($1, $2)
returning
	id;
