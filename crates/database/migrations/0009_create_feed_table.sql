CREATE TABLE IF NOT EXISTS "feed" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	name TEXT NOT NULL,

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	filter_tags TEXT[] DEFAULT '{}'::TEXT[],
	filter_profiles INTEGER[] DEFAULT '{}'::INTEGER[],
	filter_blogs INTEGER[] DEFAULT '{}'::INTEGER[],

	order_by TEXT DEFAULT 'created_at'::TEXT
);
