update "repo"
set
    blog_id = $2,
    url = $3,
    metadata = $4
where id = $1
returning id;
