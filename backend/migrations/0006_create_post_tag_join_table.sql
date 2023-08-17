CREATE TABLE IF NOT EXISTS "post_tag" (
	post_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL,
	PRIMARY KEY (post_id, tag_id),
	-- When a post is deleted, the join between the post and tag should be deleted as well.
	FOREIGN KEY (post_id) REFERENCES "post" (id) ON DELETE CASCADE,
	-- When a tag is deleted, the join between the post and tag should be deleted as well.
	FOREIGN KEY (tag_id) REFERENCES "tag" (id) ON DELETE CASCADE
);
