-- Blogs are collections of posts which belong to a profile.
CREATE TABLE IF NOT EXISTS "blog" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	profile_id UUID NOT NULL REFERENCES "profile" (id) ON DELETE CASCADE, -- A blog must belong to a profile.

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	name TEXT NOT NULL,
	slug TEXT NOT NULL UNIQUE,
	description TEXT
);
