-- $1: profile_id
-- $2: name
-- $3: slug
-- $4: description

INSERT INTO "blog" (profile_id, name, slug, description)
VALUES (
	$1, $2, $3, $4 
)
ON CONFLICT (slug) DO UPDATE SET
	profile_id = $1,
	name = $2,
	description = $4,
	updated_at = NOW()
RETURNING
	id, 
	profile_id,
	name,
	slug,
	description,
	to_char("blog".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char("blog".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at;

