INSERT INTO "post"(
  "blog_id",
  "title",
  "slug",
  "body"
)VALUES(
	(
		SELECT id AS blog_id FROM "blog"
		WHERE slug = $2
	),
$1,
$3,
$4
);
