-- $1 id
-- $2 slug
-- $3 title
-- $4 is_draft
-- $5 body

update post
set
    slug = $2,
    title = $3,
    is_draft = $4,
    body = $5
where id = $1;
