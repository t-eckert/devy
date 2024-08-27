SELECT
	id,
    blog_id,
	title,
	slug,
	body,
	COALESCE(likes.like_count, 0)::bigint AS likes,
	to_char(post.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(post.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM post
LEFT JOIN (
	SELECT post_id, COUNT(*) AS like_count
	FROM "like"
	GROUP BY post_id
) AS likes ON post.id = likes.post_id
WHERE id = $1;
