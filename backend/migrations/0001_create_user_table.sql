create table if not exists "user" (
	id uuid default uuid_generate_v4() primary key,
	profile_id uuid not null,
	email text,
	github_username text
);
