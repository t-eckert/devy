SELECT profile_id, post_id FROM "bookmark"
LEFT JOIN "profile" ON "bookmark".profile_id = "profile".id
LEFT JOIN "user" ON "profile".user_id = "user".id
WHERE "user".username = $1;
