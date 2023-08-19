-- Users are accounts that can log into the site.
CREATE TABLE IF NOT EXISTS "user" (
	id SERIAL PRIMARY KEY,
	-- A user may or may not have a profile associated with their account.
	profile_id INTEGER,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	-- A user must have a username, unique across the site.
	username TEXT NOT NULL UNIQUE,
	email TEXT NOT NULL UNIQUE,
	github_username TEXT UNIQUE,
	-- Roles are used to determine what a user can do on the site.
	-- "user" is the default role. "admin" is the highest role.
	role TEXT NOT NULL DEFAULT 'user',
	-- A user may be active or inactive. Inactive users cannot log in.
	status TEXT NOT NULL DEFAULT 'active'
);
