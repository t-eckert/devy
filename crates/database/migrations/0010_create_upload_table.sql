CREATE TABLE IF NOT EXISTS "upload" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	previous_upload_id UUID REFERENCES "upload" (id) ON DELETE NO ACTION,

	status TEXT NOT NULL DEFAULT 'pending'::TEXT,
	repo TEXT NOT NULL,
	logs TEXT[] DEFAULT '{}'::TEXT[]
);

