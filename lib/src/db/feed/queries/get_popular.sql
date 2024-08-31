SELECT
    id,
    slug AS post_slug,
    title,
    body,
    post.like_count AS likes,
    blog.blog_slug,
    blog.blog_name,
    blog.profile_slug AS author_slug,
    blog.author_name,
    TO_CHAR(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS created_at,
    TO_CHAR(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS updated_at
FROM "post" LEFT JOIN (
    SELECT
        blog.id AS blog_id,
        blog.slug AS blog_slug,
        blog.name AS blog_name,
        profile.id AS profile_id,
        profile.display_name AS author_name,
        profile_slug
    FROM "blog" LEFT JOIN (
        SELECT
            profile.id,
            profile.display_name,
            "user".username AS profile_slug
        FROM "profile" LEFT JOIN "user"
            ON user_id = "user".id
    ) "profile"
        ON profile_id = profile.id
) AS "blog" ON post.blog_id = blog.blog_id
WHERE is_draft = false
ORDER BY likes DESC;
