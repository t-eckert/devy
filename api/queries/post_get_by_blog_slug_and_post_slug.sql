SELECT 
	id::text,
	slug,
	blog_slug,
	blog_name,
	author_name,
	profile_slug AS author_slug,
	title,
	body AS content,
	to_char(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
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
WHERE blog_slug = $1 AND slug = $2;
