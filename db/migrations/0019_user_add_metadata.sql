-- Add a JSONB column to the user table to store metadata
-- This column will be used to store additional information about the user
ALTER TABLE "user"
ADD COLUMN IF NOT EXISTS metadata JSONB DEFAULT '{}';
