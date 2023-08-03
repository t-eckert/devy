create table if not exists profile {
	id uuid default uuid_generate_v4() primary key,
	user_id uuid not null,
	name text,
	avatar text,
}
