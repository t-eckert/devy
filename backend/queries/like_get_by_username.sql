SELECT profile_id, post_id
FROM "like" LEFT JOIN (
	SELECT 
		profile.id, username
	FROM "profile" LEFT JOIN "user"
	ON user_id="user".id
) AS "profile" ON profile_id="profile".id
WHERE username = $1;
