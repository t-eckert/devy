SELECT
	id, user_id, display_name,
	to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	bio, website_url AS website,
	avatar_url
FROM profile
WHERE id = $1;
