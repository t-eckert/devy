SELECT
	upload.id as id,
	previous_upload_id,
	status,
	repo,
	logs,
	to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
	to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
FROM "upload"
LEFT JOIN (
SELECT
	*
FROM
	"repo"
	INNER JOIN (
		SELECT
			id
		FROM
			"blog"
		WHERE
			profile_id = (
				SELECT
					id
				FROM
					"profile"
				WHERE
					user_id = (
						SELECT
							id
						FROM
							"user"
						WHERE
							username = $1))) AS "blog" ON blog_id = blog.id
) AS "user_repos" ON repo=user_repos.url;
