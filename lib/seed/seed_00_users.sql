-------------------------------------------------------------------------------
--- USERS ---------------------------------------------------------------------
-------------------------------------------------------------------------------

-- Add users
-- These users will have a default role of `user` and status of `active`.
INSERT INTO "user" (id, username, email, github_username)
VALUES
	('d5e7a1c3-37af-4965-83d4-5f86a9c489c4', 'techenthusiast', 'tech.enthusiast@email.com', 'techenthusiast'),
	('b7f6e4c9-0c93-4f2e-b1bf-ef07b12e6c44', 'codingpro', 'coding.pro@email.com', 'codingpro'),
	('8e7d9c2a-74aa-4d9f-af66-afcd2c6e8f8d', 'sqlgeek', 'sql.geek@email.com', 'sqlgeek'),
	('3b8d1c7e-1845-4b85-9b4c-0f63e4f29f7e', 'pythonlover', 'python.lover@email.com', 'pythonlover'),
	('f2b5e1d8-89ca-4264-918f-dc3c6f85d8ed', 'jsdeveloper', 'js.developer@email.com', 'jsdeveloper'),
	('7c4e3d6b-1d2a-43f8-8c97-8b49e3a6f4f9', 'webdesigner', 'web.designer@email.com', 'webdesigner'),
	('e6f4c8b2-8b9d-41c4-aaec-6e8d7f5d8a2e', 'datanerd', 'data.nerd@email.com', 'datanerd'),
	('1d7e2f6b-9c8d-4a6b-8d3f-6c7b1a9e2f6d', 'backenddev', 'backend.dev@email.com', 'backenddev'),
	('4b2a8e1d-8c9f-4d3a-b6e2-7f1d2e3c6b9a', 'devguru', 'dev.guru@email.com', 'devguru'),
	('3a2e6f4d-8f9e-4c1b-afec-7d8a2c9b1e2e', 'cloudadmin', 'cloud.admin@email.com', 'cloudadmin'),
	('b6c9d7a3-6f2d-4e1a-8f9d-2c1b9e8a3d7c', 'frontenddev', 'frontend.dev@email.com', 'frontenddev'),
	('9d8f7e6c-5b4a-4c3e-8d2f-1a9e2d8f7e6c', 'cybersecurity', 'cyber.security@email.com', 'cybersecurity'),
	('5c4e7b8d-1a9f-4d3c-9e6b-8f7e6c4e7b8d', 'techguru', 'tech.guru@email.com', 'techguru'),
	('2a3f4b5c-8d9e-4e6c-9b8d-1a9f2a3f4b5c', 'programmer', 'programmer@email.com', 'programmer'),
	('8c9e2a3f-4b5c-4e6d-8f7e-6c4e7b8d9e2a', 'appdeveloper', 'app.developer@email.com', 'appdeveloper'),
	('3f4b5c8d-9e2a-4e6d-7b8c-6d4e7b5c8d9e', 'codeartist', 'code.artist@email.com', 'codeartist'),
	('6c8d9e2a-4b5c-4e7f-8d9e-2a3f4b5c8d9e', 'sqlsamurai', 'sql.samurai@email.com', 'sqlsamurai'),
	('5c8d9e4b-3f2a-4e7d-9e6c-8d9e4b3f2a5c', 'pythonguru', 'python.guru@email.com', 'pythonguru'),
	('8e9d7b6c-4d3f-4a5b-9c8d-7b6c4d3f4a5b', 'javadeveloper', 'java.developer@email.com', 'javadeveloper'),
	('4d3f2a5b-8e9d-4b3f-6c7d-3f2a5b8e9d4b', 'frontendwizard', 'frontend.wizard@email.com', 'frontendwizard'),
	('9e8d7c6b-4a3f-4d5e-9e8d-7c6b4a3f4d5e', 'backendmagician', 'backend.magician@email.com', 'backendmagician'),
	('2a5b8e9d-4f3e-4c6b-7d5e-8e9d4f3e4c6b', 'fullstackninja', 'fullstack.ninja@email.com', 'fullstackninja');

INSERT INTO "user" (id, username, email, github_username, created_at, updated_at)
VALUES
	('e3d9727f-1bfd-4a56-b875-45d877ecf115', 'analogue', 'analogue@email.com', 'analogue', '2023-11-25 08:15:00+00', '2023-11-26 10:30:00+00'),
	('8f85a118-4933-4c61-8708-716ffa35cc49', 'taylorschwift', 'taylorschwift@email.com', 'taylorschwift', '2023-11-25 09:30:00+00', '2023-11-27 14:45:00+00'),
	('7cd19c80-d72b-4fc2-a818-de809361b278', 'aaronjohnson', 'a.johnson@email.com', 'ajohnson', '2023-11-25 10:45:00+00', '2023-11-28 18:00:00+00'),
	('c5f1aae6-bc85-44e9-a4c2-ad89c201fdc6', 's-miller', 'susan.miller@email.com', 'susanm', '2023-11-26 12:00:00+00', '2023-11-29 22:15:00+00'),
	('d97bdc36-cc04-4e94-91ab-ebd3bba2e0a1', 'johndoe', 'john.doe@email.com', 'johndoe', '2023-11-27 13:15:00+00', '2023-11-30 02:30:00+00'),
	('6f2db7f3-3c72-4a88-bc63-09ce924e48c4', 'janedoe', 'jane.doe@email.com', 'janedoe', '2023-11-28 14:30:00+00', '2023-12-01 06:45:00+00'),
	('af6bdc89-93ab-4c15-bf07-8d4a2a8452ce', 'codingmaster', 'coding.master@email.com', 'codingmaster', '2023-11-29 15:45:00+00', '2023-12-02 11:00:00+00'),
	('d33f21a1-52b7-4eac-9a2e-fa65fd883e9d', 'sqlwizard', 'sql.wizard@email.com', 'sqlwizard', '2023-11-30 17:00:00+00', '2023-12-03 15:15:00+00'),
	('b1e9c89d-6c5e-4c6a-a732-870c1e1e142b', 'pythonista', 'pythonista@email.com', 'pythonista', '2023-12-01 18:15:00+00', '2023-12-04 19:30:00+00'),
	('4a6e8b77-5900-4a55-82eb-026d16f3a93e', 'javascripter', 'javascripter@email.com', 'javascripter', '2023-12-02 19:30:00+00', '2023-12-05 23:45:00+00'),
	('84c07e46-1daa-4a9f-9e38-9a67c7b9836d', 'webdev', 'web.dev@email.com', 'webdev', '2023-12-03 20:45:00+00', '2023-12-06 01:00:00+00'),
	('c0f6f30a-60df-4f2e-a1db-9ff6f3a3ea39', 'dbadmin', 'db.admin@email.com', 'dbadmin', '2023-12-04 22:00:00+00', '2023-12-07 04:15:00+00'),
	('2f3d6a17-86a2-42e2-88aa-9e16d2ff42d7', 'datascientist', 'data.scientist@email.com', 'datascientist', '2023-12-05 23:15:00+00', '2023-12-08 09:30:00+00'),
	('678b5e8f-4ee6-429f-aade-426e8a86f6c4', 'devops', 'dev.ops@email.com', 'devops', '2023-12-06 00:30:00+00', '2023-12-09 14:45:00+00'),
	('1b3d6f47-7d3a-4f89-81d7-496d3fa6e98b', 'frontendguru', 'frontend.guru@email.com', 'frontendguru', '2023-12-07 01:45:00+00', '2023-12-10 20:00:00+00'),
	('a6e8c7f2-93f5-47ad-bce3-7b9ffaa5b0dd', 'backendninja', 'backend.ninja@email.com', 'backendninja', '2023-12-08 02:30:00+00', '2023-12-11 22:15:00+00'),
	('b2d8e9c4-5f5c-41e3-bb27-8f7dc9f9e7cb', 'fullstackdev', 'fullstack.dev@email.com', 'fullstackdev', '2023-12-09 03:45:00+00', '2023-12-12 01:30:00+00'),
	('0a5c4e3f-3f90-44a7-94c8-0f01db408a74', 'codelearner', 'code.learner@email.com', 'codelearner', '2023-12-10 05:00:00+00', '2023-12-13 04:45:00+00');

-- Add an admin user
INSERT INTO "user" (id, username, email, github_username, role)
VALUES
    ('20bf70c8-d1c3-4d5c-9a50-6ca87eea205c', 't-eckert', 'name@email.com', 't-eckert', 'admin');
