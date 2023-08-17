-- Bookmarks are a join on profiles and posts.
CREATE TABLE IF NOT EXISTS "bookmark" (
	profile_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL,
	PRIMARY KEY (profile_id, post_id),
	-- When a profile is deleted, its bookmarks should be deleted as well.
	FOREIGN KEY (profile_id) REFERENCES "profile" (id) ON DELETE CASCADE,
	-- When a post is deleted, its bookmarks should be deleted as well.
	FOREIGN KEY (post_id) REFERENCES "post" (id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
