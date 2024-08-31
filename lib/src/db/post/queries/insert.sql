INSERT INTO "post" (blog_id, title, slug, body)
VALUES ($1, $2, $3, $4)
RETURNING
  id,
  blog_id,
  title,
  slug,
  body,
  to_char("post".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
  to_char("post".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
  0::bigint AS likes,
  is_draft;
