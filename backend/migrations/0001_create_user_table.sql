-- Users are accounts that can log into the site.
CREATE TABLE IF NOT EXISTS "user" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	username TEXT NOT NULL UNIQUE, -- A user must have a username, unique across the site.
	email TEXT NOT NULL UNIQUE,
	github_username TEXT UNIQUE,
	role TEXT NOT NULL DEFAULT 'user', -- Roles are used to determine what a user can do on the site.
	status TEXT NOT NULL DEFAULT 'active' 	-- A user may be active or inactive. Inactive users cannot log in.
);
