ALTER TABLE "blog" ADD COLUMN "updated_at" timestamp;--> statement-breakpoint
ALTER TABLE "blog" ADD COLUMN "created_at" timestamp DEFAULT now() NOT NULL;--> statement-breakpoint
ALTER TABLE "post" ADD COLUMN "updated_at" timestamp;--> statement-breakpoint
ALTER TABLE "post" ADD COLUMN "created_at" timestamp DEFAULT now() NOT NULL;