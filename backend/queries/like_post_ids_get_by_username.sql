SELECT 
	post_id::TEXT, profile_id::TEXT 
FROM "like" LEFT JOIN (
	SELECT 
		profile.id, username
	FROM "profile" LEFT JOIN "user"
	ON user_id="user".id
) AS "profile" ON profile_id="profile".id
WHERE username = $1;
