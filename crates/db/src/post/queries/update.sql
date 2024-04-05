INSERT INTO post(id, blog_id, title, slug, body)VALUES(
  $1,
	(
		SELECT id AS blog_id FROM "blog"
		WHERE slug = $2
	),
  $3,
  $4,
  $5
)
ON CONFLICT (id) DO UPDATE SET 
  blog_id = (
  SELECT
    id AS blog_id
  FROM
    "blog"
  WHERE
    slug = $2
  ),
title = $3,
slug = $4,
body = $5,
updated_at = NOW();
