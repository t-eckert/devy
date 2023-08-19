CREATE TABLE IF NOT EXISTS "post" (
	id SERIAL PRIMARY KEY,
	-- A post must belong to a blog.
	blog_id INTEGER NOT NULL,
	-- A post must belong to a profile.
	profile_id INTEGER NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	-- A post must have a title.
	title TEXT NOT NULL,
	-- A post must have a slug.
	slug TEXT NOT NULL UNIQUE,
	-- A post must have a body.
	body TEXT NOT NULL
);
