-- Blogs are collections of posts which belong to a profile.
CREATE TABLE IF NOT EXISTS "blog" (
	id SERIAL PRIMARY KEY,
	-- A blog must belong to a profile.
	profile_id INTEGER NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	name TEXT,
	slug TEXT NOT NULL UNIQUE
);
