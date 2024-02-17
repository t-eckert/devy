INSERT INTO "user" (username, email, github_username, role, status) 
VALUES ($1, $2, $3, COALESCE($4, 'user'), COALESCE($5, 'active'))
ON CONFLICT (username) DO UPDATE SET
	email = $2,
	github_username = $3,
	role = COALESCE($4, 'user'),
	status = COALESCE($5, 'active'),
	updated_at = NOW()
RETURNING 
	id, 
	username,
	email,
	github_username,
	role,
	status,
	to_char("user".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char("user".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;
