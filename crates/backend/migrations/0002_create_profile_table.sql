CREATE TABLE IF NOT EXISTS "profile" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

	user_id UUID NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

	display_name TEXT,
	bio TEXT,
	avatar_url TEXT,
	website_url TEXT,
	twitter_username TEXT,
	github_username TEXT,
	
	status TEXT NOT NULL DEFAULT 'active', -- May be active or inactive. Inactive profiles cannot be viewed.
	visibility TEXT NOT NULL DEFAULT 'public', -- May be public or private. Private profiles cannot be viewed.

	is_deleted BOOLEAN NOT NULL DEFAULT FALSE,	-- May be deleted or not deleted. Deleted profiles cannot be viewed.
	is_locked BOOLEAN NOT NULL DEFAULT FALSE,	-- May be locked or unlocked. Locked profiles cannot be edited.
	is_featured BOOLEAN NOT NULL DEFAULT FALSE,	-- May be featured or not featured. Featured profiles are displayed on the home page.
	is_bot BOOLEAN NOT NULL DEFAULT FALSE,		-- May be a bot or not a bot. Bots are not displayed on the home page.
	is_sponsor BOOLEAN NOT NULL DEFAULT FALSE	-- May be a sponsor or not a sponsor. Sponsors are displayed on the home page.
);


