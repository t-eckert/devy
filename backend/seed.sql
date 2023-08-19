INSERT INTO "user" (id, profile_id, username, email, github_username)
VALUES (1, 1, 't-eckert', 'name@email.com', 't-eckert');

INSERT INTO "profile" (id, user_id, display_name, bio, avatar_url, twitter_username, github_username)
VALUES (1, 1, 'Tobias Eckert', 'I am a software engineer.', 'https://avatars.githubusercontent.com/u/1234567?v=4', 't_eckert', 't-eckert');

INSERT INTO "blog" (id, profile_id, name, slug)
VALUES (1, 1, 'Machine Learning', 'machine-learning');
