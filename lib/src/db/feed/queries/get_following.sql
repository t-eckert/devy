SELECT
	post.id as id,
	post.slug as post_slug,
	title,
	body,
	COALESCE(likes.like_count, 0)::bigint AS likes,
	blog.slug as blog_slug,
	blog.name as blog_name,
	"user".username as author_slug,
	profile.display_name as author_name,
	to_char(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS created_at,
	to_char(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss"Z"') AS updated_at
FROM "follow"
	JOIN "post" ON follow.blog_id = post.blog_id
 	JOIN "blog" ON post.blog_id = blog.id
 	JOIN "profile" ON blog.profile_id = profile.id
 	JOIN "user" ON profile.user_id = "user".id
	LEFT JOIN (
		SELECT post_id, COUNT(*) AS like_count
		FROM "like"
		GROUP BY post_id
	) AS likes ON post.id = likes.post_id
ORDER BY post.created_at DESC;
