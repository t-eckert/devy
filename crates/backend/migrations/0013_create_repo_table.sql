/* REPO

A repo is almost like a join table between `Blog` and `Upload`. This relation
allows all of the uploads to connect up to a single `Repo`. 
*/

CREATE TABLE IF NOT EXISTS repo (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    url TEXT NOT NULL UNIQUE,
    blog_id UUID NOT NULL UNIQUE,

    github_id BIGINT NOT NULL UNIQUE,
    github_name TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
