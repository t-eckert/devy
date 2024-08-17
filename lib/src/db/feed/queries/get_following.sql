SELECT
    id,
    slug as post_slug,
	title,
	body,
	COALESCE(likes.like_count, 0)::bigint AS likes,
	blog.blog_slug,
	blog.blog_name,
	blog.profile_slug as author_slug,
	blog.author_name,
	to_char(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS created_at,
	to_char(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS updated_at
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
			"user".username as profile_slug
		FROM "profile" LEFT JOIN "user"
		ON user_id = "user".id
	) "profile"
	ON profile_id=profile.id
) AS "blog" ON post.blog_id = blog.blog_id
LEFT JOIN (
	SELECT post_id, COUNT(*) AS like_count
	FROM "like"
	GROUP BY post_id
) AS likes ON post.id = likes.post_id
ORDER BY post.created_at DESC;
