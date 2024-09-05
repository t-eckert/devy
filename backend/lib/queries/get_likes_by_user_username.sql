SELECT profile_id, post_id FROM "like"
LEFT JOIN "profile" ON "like".profile_id = "profile".id
LEFT JOIN "user" ON "profile".user_id = "user".id
WHERE "user".username = $1;
