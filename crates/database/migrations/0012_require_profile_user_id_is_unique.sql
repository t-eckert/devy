ALTER TABLE "profile"
DROP CONSTRAINT IF EXISTS "unique_user_id";
ALTER TABLE "profile"
ADD CONSTRAINT "unique_user_id" UNIQUE (user_id);
