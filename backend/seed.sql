INSERT INTO "user" (id, profile_id, username, email, github_username)
VALUES 
	(1, 1, 't-eckert', 'name@email.com', 't-eckert'),
	(2, null, 't-admin', 'admin@email.com', null),
	(3, 2, 'analogue', 'analogue@email.com', 'analogue'),
	(4, 3, 'taylorschwift', 'taylorschwift@email.com', 'taylorschwift'),
	(5, 4, 'aaronjohnson', 'a.johnson@email.com', 'ajohnson'),
	(6, 5, 's-miller', 'susan.miller@email.com', 'susanm');


INSERT INTO "profile" (id, user_id, display_name, bio, avatar_url, twitter_username, github_username)
VALUES 
    (1, 1, 'Tobias Eckert', 'I am a software engineer.', 'https://avatars.githubusercontent.com/u/1234567?v=4', 't_eckert', 't-eckert'),
    (2, 3, 'Anna Logue', 'I like to build backend stuff.', null, null, null),
    (3, 4, 'Taylor Schwift', 'Coding enthusiast and traveler.', 'https://avatars.example.com/johnsmith', 'ts_coder', 'taylorschwift'),
    (4, 5, 'Aaron Johnson', 'Loves writing and sharing stories.', 'https://avatars.example.com/marydoe', null, 'ajohnson'),
    (5, 6, 'Susan Miller', 'Web developer and adventure seeker.', 'https://avatars.example.com/alicej', 'alice_dev', 'susanm');


INSERT INTO "blog" (id, profile_id, name, slug)
VALUES 
	(1, 1, 'Machine Learning', 'machine-learning'),
	(2, 1, 'DevOops', 'devoops');
