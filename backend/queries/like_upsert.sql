INSERT INTO "like" (profile_id, post_id)
VALUES ($1, $2)
	ON CONFLICT (profile_id, post_id)
	DO UPDATE SET profile_id = $1, post_id = $2
RETURNING profile_id, post_id;
