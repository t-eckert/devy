CREATE TABLE IF NOT EXISTS "post" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	blog_id UUID NOT NULL REFERENCES "blog" (id) ON DELETE CASCADE, -- must belong to a blog

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	title TEXT NOT NULL,		-- must have a title
	slug TEXT NOT NULL UNIQUE,	-- must have a slug
	body TEXT NOT NULL			-- must have a body, formatted in Markdown
);
