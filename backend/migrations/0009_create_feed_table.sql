CREATE TABLE IF NOT EXISTS "feed" (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	filter_tags TEXT[] DEFAULT '{}'::TEXT[],
	filter_profiles INTEGER[] DEFAULT '{}'::INTEGER[],
	filter_blogs INTEGER[] DEFAULT '{}'::INTEGER[],
	order_by TEXT DEFAULT 'created_at'::TEXT
);
