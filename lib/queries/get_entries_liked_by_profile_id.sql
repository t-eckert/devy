SELECT
   *
FROM "like"
LEFT JOIN "post" ON "like".post_id = "post".id
WHERE profile_id = $1;
