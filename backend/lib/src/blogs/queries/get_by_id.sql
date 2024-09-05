SELECT
	blog.id as id,
	profile.id as profile_id,
	user_id,
	name,
	slug,
	description,
	to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM "blog" LEFT JOIN (
	SELECT
		profile.id, display_name, username, "user".id as user_id
	FROM "profile" LEFT JOIN "user"
	ON user_id="user".id
) AS "profile" ON profile_id="profile".id
WHERE blog.id=$1;
