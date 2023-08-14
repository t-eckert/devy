CREATE TABLE IF NOT EXISTS "user" (
	id SERIAL PRIMARY KEY,
	profile_id INT,
	username TEXT NOT NULL UNIQUE,
	email TEXT,
	github_username TEXT
);
