ALTER TABLE "profile" ADD COLUMN "updated_at" timestamp;--> statement-breakpoint
ALTER TABLE "profile" ADD COLUMN "created_at" timestamp DEFAULT now() NOT NULL;