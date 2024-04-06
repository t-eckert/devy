CREATE TABLE IF NOT EXISTS "session" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,

    metadata JSONB NOT NULL DEFAULT '{}',

    -- The session token
    token TEXT NOT NULL UNIQUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    exp INTEGER NOT NULL DEFAULT 3600,

    access_token TEXT NOT NULL,
    encoding_key TEXT NOT NULL
);
