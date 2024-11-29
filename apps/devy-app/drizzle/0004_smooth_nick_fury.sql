ALTER TABLE "blog" ADD COLUMN "profileId" uuid NOT NULL;--> statement-breakpoint
ALTER TABLE "profile" ADD COLUMN "username" text NOT NULL;