-- Change the default value of the upload sha to be HEAD instead of an empty string.
-- This is to ensure that the upload is always associated with the latest commit unless
-- the user specifies a different commit.
ALTER TABLE upload ALTER COLUMN sha SET DEFAULT 'HEAD';
