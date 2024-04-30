-- Adds a SHA column to the upload table.
ALTER TABLE "upload"
ADD COLUMN IF NOT EXISTS sha TEXT NOT NULL DEFAULT '';
