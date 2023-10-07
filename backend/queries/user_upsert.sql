INSERT INTO "user" (username, email, github_username)
VALUES ($1, $2, $3)
ON CONFLICT (username) DO UPDATE SET
	email = EXCLUDED.email,
	github_username = EXCLUDED.github_username,
	updated_at = now()
RETURNING 
	id::TEXT,
	to_char("user".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char("user".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	username,
	email, 
	github_username, 
	role,
	status;
