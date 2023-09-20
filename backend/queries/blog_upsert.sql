INSERT INTO "blog" ("profile_id", "name", "slug", "description")
VALUES (
	( 
		SELECT id AS profile_id FROM "profile" 
		WHERE user_id=(SELECT id from "user" WHERE username=$3)
	),
	$1, $2, $4
);
