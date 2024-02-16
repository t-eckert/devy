DELETE FROM "like"
WHERE profile_id = $1 AND post_id = $2;
