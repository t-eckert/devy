INSERT INTO profile (user_id, display_name, avatar_url)
VALUES ($1, $2, $3)
ON CONFLICT (user_id) DO UPDATE SET
	display_name = EXCLUDED.display_name,
	avatar_url = EXCLUDED.avatar_url,
	updated_at = NOW()
RETURNING 
	id::TEXT, user_id::TEXT, display_name,
	to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	bio, website_url AS website,
	avatar_url;
