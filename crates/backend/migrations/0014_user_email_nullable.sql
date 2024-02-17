-- Make user.email nullable, this mitigates the issue of a user attempting to
-- sign up with GitHub when their GitHub account does not have an email address.
-- In the frontend, we will check for a null email and prompt the user to enter
-- an email address for them to be able to access any email-related features.
ALTER TABLE "user" ALTER COLUMN email DROP NOT NULL;

