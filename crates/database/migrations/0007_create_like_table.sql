-- Likes are a join on profiles and posts.
CREATE TABLE IF NOT EXISTS "like" (
	profile_id UUID NOT NULL REFERENCES "profile" (id) ON DELETE CASCADE,
	post_id UUID NOT NULL REFERENCES "post" (id) ON DELETE CASCADE,
	PRIMARY KEY (profile_id, post_id),

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
