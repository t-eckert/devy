-- Follows are a join on profiles and blogs.
CREATE TABLE IF NOT EXISTS "follow" (
    profile_id UUID NOT NULL REFERENCES "profile" (id) ON DELETE CASCADE,
	blog_id UUID NOT NULL REFERENCES "blog" (id) ON DELETE CASCADE,
	PRIMARY KEY (profile_id, blog_id),

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
