-- $1: id
-- $2: blog_id
-- $3: title
-- $4: slug
-- $5: body

WITH Upserted AS (
    INSERT INTO "post" (id, blog_id, title, slug, body)
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (id) DO UPDATE SET
        blog_id = EXCLUDED.blog_id,
        title = EXCLUDED.title,
        slug = EXCLUDED.slug,
        body = EXCLUDED.body,
        updated_at = NOW()
    RETURNING id
)
SELECT
    p.id,
    p.blog_id,
    p.title,
    p.slug,
    p.body,
    to_char(p.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
    to_char(p.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
    COALESCE(l.like_count, 0) AS likes,
    is_draft
FROM post p
LEFT JOIN (
    SELECT post_id, COUNT(*) AS like_count
    FROM "like"
    GROUP BY post_id
) l ON p.id = l.post_id
WHERE p.id = $1;
