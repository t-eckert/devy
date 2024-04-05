-- $1: id
-- $2: blog_id
-- $3: title
-- $4: slug
-- $5: body

INSERT INTO "post" (blog_id, title, slug, body)
VALUES ($2, $3, $4, $5)
ON CONFLICT (id) DO UPDATE SET
  blog_id = $2,
  title = $3,
  slug = $4,
  body = $5,
  updated_at = NOW()
RETURNING
  id,
  blog_id,
  title,
  slug,
  body,
  to_char("post".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
  to_char("post".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
  10 AS likes;
