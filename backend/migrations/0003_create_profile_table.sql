CREATE TABLE IF NOT EXISTS "profile" (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL UNIQUE,
	display_name TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	bio TEXT,
	avatar_url TEXT,
	website_url TEXT,
	twitter_username TEXT,
	github_username TEXT,
	-- A profile may be active or inactive. Inactive profiles cannot be viewed.
	status TEXT NOT NULL DEFAULT 'active',
	-- A profile may be public or private. Private profiles cannot be viewed.
	visibility TEXT NOT NULL DEFAULT 'public',
	-- A profile may be deleted or not deleted. Deleted profiles cannot be viewed.
	id_deleted BOOLEAN NOT NULL DEFAULT FALSE,
	-- A profile may be locked or unlocked. Locked profiles cannot be edited.
	is_locked BOOLEAN NOT NULL DEFAULT FALSE,
	-- A profile may be featured or not featured. Featured profiles are displayed on the home page.
	is_featured BOOLEAN NOT NULL DEFAULT FALSE,
	-- A profile may be a bot or not a bot. Bots are not displayed on the home page.
	is_bot BOOLEAN NOT NULL DEFAULT FALSE,
	-- A profile may be a sponsor or not a sponsor. Sponsors are displayed on the home page.
	is_sponsor BOOLEAN NOT NULL DEFAULT FALSE
);


