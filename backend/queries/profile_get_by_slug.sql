SELECT 
	to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	display_name, avatar_url
FROM profile LEFT JOIN "user" ON profile.user_id = "user".id
WHERE username=$1;
