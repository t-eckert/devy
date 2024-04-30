SELECT 
	id,
	to_char("user".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char("user".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
	username, email, github_username, role, status
FROM "user" 
WHERE username=$1;
