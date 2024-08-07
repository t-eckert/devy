SELECT
	id,
	post.blog_id,
	COALESCE(slug, '') AS slug,
	title,
	body,
	to_char(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	COALESCE(likes.like_count, 0) AS likes
FROM "post" INNER JOIN
	(SELECT * FROM "like" WHERE profile_id =
		(SELECT id FROM "profile" WHERE user_id =
			(SELECT id FROM "user" WHERE username = $1)))
AS user_likes ON "post".id = post_id
LEFT JOIN (
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
INNER JOIN (
	SELECT post_id, COUNT(*) AS like_count
	FROM "like"
	GROUP BY post_id
) AS likes ON post.id = likes.post_id;
