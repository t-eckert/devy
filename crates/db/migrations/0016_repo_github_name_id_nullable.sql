-- Make repo.github_name and repo.github_id nullable. This information will be available
-- in the metadata row.
ALTER TABLE "repo" ALTER COLUMN "github_name" DROP NOT NULL;
ALTER TABLE "repo" ALTER COLUMN "github_id" DROP NOT NULL;
