SELECT 
	blog.id as id,
	name, 
	slug,
	username, 
	display_name, 
	description,
	to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM "blog" LEFT JOIN (
	SELECT 
		profile.id, display_name, username
	FROM "profile" LEFT JOIN "user"
	ON user_id="user".id
) AS "profile" ON profile_id="profile".id
WHERE slug=$1;
