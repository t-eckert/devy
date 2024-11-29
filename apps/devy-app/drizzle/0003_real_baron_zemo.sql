CREATE TABLE IF NOT EXISTS "post" (
	"id" uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
	"blogId" uuid NOT NULL,
	"name" text NOT NULL
);
